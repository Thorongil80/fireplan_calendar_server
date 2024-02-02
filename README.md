# fireplan_alarm_imap

## English
Pull IMAP accounts using IMAP IDLE, evaluate email text, create alarm via Fireplan API

Put config file fireplan_alarm_imap.conf to the %USERPROFILE% directory of the user running the program (Windows) or into user home ~ (Unixoid).

You build the program using "cargo" after having installed Rust toolchain from rustup.rs web site. "cargo build -r" in the cloned repository. Find executable below target/release/ subdirectory.

Find example template (TOML format) in the repository. For each "Standort", create a separate section with imap and fireplan connection details.

install.sh script compiles and installs as systemd service on boot.

Adapt fireplan_alarm_imap.service file to your needs.
By default service runs automatically after boot as user "admin" after network is up and in multi-user mode.
This makes it run e.g. in a default installation of Raspberry Pi OS.

Adapt WantedBy to run it e.g. in single-user mode as well if your machine is not booting through to multi-user.
Adapt user name to your needs. Attention: put config file to this user's home directory!

## Deutsch
Eine kleine Anwendung, die IMAP accounts mit IMAP IDLE nach neuen Emails abfragt, den Email-Text auswertet und einen Alarm an die API von Fireplan übergit.

Man legt dazu die Konfigurationsdatei in das Verzeichnis %USERPROFILE% (Windows) bzw. ~ (Unixe) des Benutzers unter dem das Programm läuft.

Das Programm baut man mit "cargo" nachdem Rust installiert ist. "cargo build -r" im geklonten Verzeichnis. Dann leigt fireplan_alarm_imap(.exe) im Unterverzeichnis target/release/
Rust installiert man von der Webseite rustup.rs.

Eine Beispielkonfiguration (im TOML-Format) ist enthalten. Für jeden "Standort" legt man eine separate Sektion mit IMAP und Fireplan Verbindungsdetails an.

Das skript install.sh compiliert und installiert das tool als systemd service beim Booten (Linux).

Die Datei fireplan_alarm_imap.service muss dazu angepasst werden. Standardmäßig läuft es als Benutzer "admin" nachdem das Netzwerk gestartet ist im Multi-User-Modus. Das passt so für Raspberry Pi OS zum Beispiel. Achtung: die Konfigurationsdatei muss dann natürlich in dieses Benutzers home Verzeichnis liegen (/home/admin z.B.).

Man muss "WantedBy" entsprechend anpassen, wenn das System nicht in den Multi-User-Modus sondenr ein anderes Runlevel (so sagte man früher dazu) bootet (relativ selten).
