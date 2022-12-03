cd ../newlink
cargo lambda build --release && cd - && terraform apply --auto-approve
