/* Copyright 2025 Lucas A. Paul <lucas.a.paul@proton.me>
 * This file is part of the Racket extension for Zed which is released under the Apache 2.0 license.
 * See LICENSE or go to http://www.apache.org/licenses/ for full license details.
 */

use zed_extension_api::{self as zed, Command, Extension, LanguageServerId, Worktree};

struct RacketExtension {}

impl Extension for RacketExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Command> {
        Ok(Command {
            command: worktree
                .which("racket")
                .ok_or("Unable to find racket binary. Is Racket installed and in PATH?")?,
            args: vec!["-l".to_string(), "racket-langserver".to_string()],
            env: Vec::new(),
        })
    }
}

zed::register_extension!(RacketExtension);
