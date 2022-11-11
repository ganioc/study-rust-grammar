use clap::{Parser, Args, Subcommand, ValueEnum};
use anyhow::Result;

#[derive(Parser)]
#[command(CMD ATTRIBUTE)]
#[group(GROUP ATTRIBUTE)]

struct Cli {
    #[arg(ARG ATTRIBUTE)]
    field: UserType,

    #[arg(value_enum, ARG ATTRIBUTE...)]
    field: EnumValues,

    #[command(flatten)]
    delegate: Struct,

    #[command(subcommand)]
    command: Command,
}

fn get_cluster_info() -> Result

