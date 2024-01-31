#!/bin/bash

set -euxo pipefail

echo "Building..."
cargo build -r
echo "If running, stop systemd service..."
sudo systemctl stop fireplan_alarm_imap || true
echo "Copying to /usr/local/bin"
sudo cp target/release/fireplan_alarm_imap /usr/local/bin
echo "Installing systemd service..."
sudo cp fireplan_alarm_imap.service /etc/systemd/system/
echo "Reloading systemd daemons..."
sudo systemctl daemon-reload
echo "Enabling systemd service on boot..."
sudo systemctl enable fireplan_alarm_imap --now
echo "Installation done"
