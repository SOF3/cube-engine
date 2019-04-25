/*
 * cube-engine
 *
 * Copyright (C) 2019 SOFe
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#![allow(dead_code)]

extern crate heck;
extern crate quote;
extern crate rustfmt;
extern crate syn;

use std::collections::HashMap;
use std::env::var;
use std::fs::File;
use std::path::Path;

use heck::CamelCase;
use heck::SnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use ::rustfmt::config::Config;

use crate::protocol::signal::SignalDirection;
use crate::protocol::spec::Spec;
use crate::protocol::types::Type;
use self::rustfmt::{Input, format_input};
use std::io::Write;

mod spec;
mod fsm;
mod types;
mod signal;
mod format;

fn ident(ident: &String) -> Ident { Ident::new(ident.as_str(), Span::call_site()) }

fn ident_str(ident: &str) -> Ident { Ident::new(ident, Span::call_site()) }

pub type SignalId = u16;
pub const SIGNAL_ID_TYPE: &'static str = "u16";

pub fn generate() {
    let spec = Spec::new();

    // validates that there are no duplicate signal IDs
    {
        let map = HashMap::<SignalId, String>::new();
        for (_, group) in &spec.groups {
            for signal in &group.signals {
                let id = signal.signal_id();
                if map.contains_key(&id) {
                    panic!("Signals have duplicate IDs: {}, {}", map[&id], signal.name);
                }
            }
        }
    }

    let out_dir = var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("protocol.rs");

    {
        let states: Vec<_> = spec.fsm.states.iter().map(|(name, _)| ident(&name.to_camel_case()))
            .collect();

        let fsm_states = quote! {
            #[derive(Debug)]
            pub enum ConnectionState{
                #(#states),*
            }
        };

        let pub_types = spec.types.iter().map(|(name, typ)| {
            let type_struct = ident(&name.to_camel_case());
            let fields = quote_fields(typ);
            quote! {
                pub struct #type_struct {
                    #(#fields)*
                }
            }
        });
        let signal_types: Vec<TokenStream> = spec.groups.iter().flat_map(|(_, group)| {
            let v: Vec<TokenStream> = group.signals.iter().map(|signal| {
                let id = signal.signal_id();
                let id_name = ident(&signal.signal_id_name());
                let type_name = ident(&signal.signal_type());
                let fields = quote_fields(&signal.fields);
                quote! {
                    pub const #id_name: SignalId = #id;
                    pub struct #type_name { #(#fields)* }
                }
            }).collect();
            v
        }).collect();


        let client_signals = generate_endpoint_impl(&spec, false);
        let server_signals = generate_endpoint_impl(&spec, true);

        let mut f = File::create(&dest_path).unwrap();
        let signal_id_type = ident(&SIGNAL_ID_TYPE.to_owned());
        let code = quote! {
            pub type SignalId = #signal_id_type;
            #fsm_states
            #(#pub_types)*
            #(#signal_types)*
            #(#client_signals)*
            #(#server_signals)*
        }.to_string();

//        let text = Input::Text(code);
//        let config = Config::default();
//        format_input(text, &config, Some(&mut f)).unwrap();

        f.write_all(code.as_bytes()).unwrap();
    }
}

fn generate_endpoint_impl(spec: &Spec, server: bool) -> TokenStream {
    let signals: Vec<_> = spec.groups.iter().flat_map(|(group_name, group)| {
        group.signals.iter().flat_map(move |signal| {
            let mut quotes = Vec::<TokenStream>::new();

            let (write, read) = match signal.direction {
                SignalDirection::ClientToServer => (!server, server),
                SignalDirection::ServerToClient => (server, !server),
                SignalDirection::Mutual => (true, true),
            };

            let description = &signal.description;
            let signal_type_name = signal.signal_type();
            let signal_type = ident(&signal_type_name);
            let state_trans: Vec<TokenStream> = spec.fsm.states
                .iter().flat_map(|(_, from_state)| {
                let ret: Vec<TokenStream> = from_state.edges.iter().filter_map(|edge| {
                    if edge.group_name != group_name.to_owned() { return None; }
                    let from_name = ident(&from_state.name.to_camel_case());
                    let to_name = ident(&edge.to_name.to_camel_case());
                    Some(quote! {
                            ConnectionState::#from_name => ConnectionState::#to_name
                        })
                }).collect();
                ret
            }).collect();
            if write {
                let signal_write_method = ident(&format!("write_{}", &signal.name.to_snake_case()));
                let state_trans = state_trans.clone();
                let write_code = signal.fields.iter().map(|(field_name, format)| {
                    let field_name = ident(field_name);
                    let format = format.write_code();
                    quote! {{
                            let target = &mut self.#field_name;
                            #format
                    }}
                }).collect();
                quotes.push(quote! {
                    #[doc = #description]
                    fn #signal_write_method(&mut self, signal: #signal_type) -> NetRet<()> {
                        let state = self.state;
                        self.state = match state {
                            _  => make_err(format!("Signal {} not allowed at state {:#?}", #signal_type_name, state))?,
                            #(#state_trans),*
                        };
                        #(#write_code)*
                        Ok(())
                    }
                })
            }
            if read {
                let signal_read_method = ident(&format!("read_{}", &signal.name.to_snake_case()));
                quotes.push(quote! {
                    #[doc = #description]
                    fn #signal_read_method(&mut self) -> NetRet<#signal_type> {
                        let state = self.state;
                        self.state = match state {
                            _ => make_err(format!("Signal {} not allowed at state {}", #signal_type_name, state))?,
                            #(#state_trans),*
                        };
                        Ok(())
                    }
                })
            }
            quotes
        })
    }).collect();

    let endpoint_name = ident(&(if server { "Server" } else { "Client" }).to_owned());
    quote! {
        impl <W: Write, R: Read> #endpoint_name<W, R> {
            #(#signals)*
        }
    }
}

fn quote_fields(typ: &Type) -> Vec<TokenStream> {
    typ.iter().map(|(name, format)| {
        if name.starts_with("_nop_") {
            quote! {  }
        }else{
            let name = ident(&name.to_snake_case());
            let format = format.return_type();
            quote! { #name: #format, }
        }
    }).collect()
}
