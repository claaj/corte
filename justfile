# Corte

build:
	@cargo build --release

clean:
	@rm -r target

install:
    @sudo cp resources/corte.conf /usr/lib/tmpfiles.d/
    @sudo cp resources/corte.service /usr/lib/systemd/user/
    @sudo cp target/release/corte /usr/bin/
    @systemctl enable --now corte --user

uninstall:
    @systemctl disable --now corte --user
    @sudo rm /usr/lib/tmpfiles.d/corte.conf
    @sudo rm /usr/lib/systemd/user/corte.service
    @sudo rm /usr/bin/corte

reinstall: clean build uninstall install
