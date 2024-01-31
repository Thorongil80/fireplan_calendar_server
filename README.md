# fireplan_alarm_imap
Pull IMAP accounts using IMAP IDLE, evaluate email text, create alarm via Fireplan API

Put config file fireplan_alarm_imap.conf to the %USERPROFILE% directory of the user running the program (Windows) or into user home ~ (Unixoid).

Find example template in the repository.

install.sh script installs as systemd service on boot.

Adapt fireplan_alarm_imap.service file to your needs.
By default service runs automatically after boot as user "admin" after network is up and in multi-user mode.

Adapt target.wants to run it in single-user as well if your machine is not booting through to multi-user.
Adapt user name to your needs. Attention: put config file to this user's home directory!
