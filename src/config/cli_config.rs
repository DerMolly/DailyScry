/*
 * SPDX-FileCopyrightText: 2024 Philip Molares <philip.molares@udo.edu>
 *
 * SPDX-License-Identifier: MIT
 */

use clap::Parser;
use clap_verbosity_flag::Verbosity;

const HELP_TEMPLATE: &str = "\
{before-help}{about-with-newline}

{usage-heading} {usage}

{all-args}

Version: {version}

Author: {author-with-newline}
{after-help}";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, help_template = HELP_TEMPLATE)]
pub struct CLIConfig {
    #[clap(flatten)]
    pub loglevel: Verbosity,

    #[arg(long, help = "Post to mastodon")]
    pub mastodon: bool,

    #[arg(long, help = "Post to telegram")]
    pub telegram: bool,

    #[arg(long, help = "Run the command without posting anything")]
    pub dry_run: bool,
}

impl CLIConfig {
    pub fn new() -> Self {
        let cli = CLIConfig::parse();
        pretty_env_logger::formatted_builder()
            .filter_level(cli.loglevel.log_level_filter())
            .init();
        cli
    }
}
