use clap::Args;

#[derive(Args)]
#[command(about = "Get the next road tax date for a car")]
pub struct NextRoadTax {
    #[arg(short, long)]
    pub plate: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found() {}

    #[test]
    fn test_success() {}
}
