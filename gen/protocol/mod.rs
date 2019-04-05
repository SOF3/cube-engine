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
extern crate syn;

use std::env::var;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use heck::CamelCase;
use heck::SnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::protocol::signal::SignalDirection;
use crate::protocol::spec::Spec;

mod spec;
mod fsm;
mod types;
mod signal;
mod format;

fn ident(ident: &String) -> Ident { Ident::new(ident.as_str(), Span::call_site()) }

pub fn generate() {
    let spec = Spec::new();

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

        let pub_types = spec.types.map.iter().map(|(name, typ)| {
            let type_struct = ident(&name.to_camel_case());
            let fields: Vec<String> = vec![];
            quote! {
                pub struct #type_struct {
                    #(#fields),*
                }
            }
        });

        let client_signals = generate_endpoint_impl(&spec, false);
        let server_signals = generate_endpoint_impl(&spec, true);

        let mut f = File::create(&dest_path).unwrap();
        f.write_all(quote! {
            #fsm_states
            #(#pub_types)*
            #(#client_signals)*
            // Server signals
            #(#server_signals)*
        }.to_string().as_bytes()).unwrap();
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

            if write {
                let signal_type_name = signal.name.to_camel_case();
                let signal_type = ident(&signal_type_name);
                let signal_write_method = ident(&format!("write_{}", &signal.name.to_snake_case()));
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
                let description = &signal.description;
                quotes.push(quote! {
                    #[doc = #description]
                    fn #signal_write_method(&mut self, signal: #signal_type) -> NetRet<()> {
                        let state = self.state;
                        self.state = match state {
                            _  => make_err(format!("Signal {} not allowed at state {}", #signal_type_name, state))?,
                            #(#state_trans),*
                        };
                        Ok(())
                    }
                })
            }
            if read {
                quotes.push(quote! {})
            }

            quotes
        })
    }).collect();

    let endpoint_name = ident(&(if server { "Server" } else { "Client" }).to_owned());
    quote! {
        impl #endpoint_name {
            #(#signals)*
        }
    }
}
