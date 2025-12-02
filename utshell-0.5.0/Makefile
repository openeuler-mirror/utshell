prefix = /usr/bin
bindir = /usr/bin
translationdir = /usr/share/utshell/resources

INSTALL = /usr/bin/install -c
INSTALL_DATA = ${INSTALL} -m 644
INSTALLMODE = -m 0755
INSTALLMODE2 = -m 0555

TOP_DIR = $(PWD)
LANGSRC = resources
BIN_DIR = target/debug
CONFIG_DIR = config

LIB_DIR := $(TOP_DIR)/variable \
		   $(TOP_DIR)/lib/glob \
		   $(TOP_DIR)/lib/readline \
		   $(TOP_DIR)/lib/sh \
		   $(TOP_DIR)/lib/termcap \
		   $(TOP_DIR)/lib/tilde


.PHONY:all
all:build_lib build_utshell
build_lib:
	-for libdir in ${LIB_DIR}; do \
		(cd $$libdir && make) ;\
	done
build_utshell:
	cargo build

install:
	mkdir -p $(DESTDIR)$(bindir)
	mkdir -p $(DESTDIR)/root
	mkdir -p $(DESTDIR)/etc
	install -m 0755 $(BIN_DIR)/utshell $(DESTDIR)$(bindir)/utshell
	install -m 0644 $(CONFIG_DIR)/.utshellrc 		$(DESTDIR)/root/.utshellrc
	install -m 0644 $(CONFIG_DIR)/.utshell_profile 	$(DESTDIR)/root/.utshell_profile
	install -m 0644 $(CONFIG_DIR)/.utshell_logout 	$(DESTDIR)/root/.utshell_logout 
	install -m 0644 $(CONFIG_DIR)/utshellrc 	    $(DESTDIR)/etc/utshellrc 
	-( $(INSTALL_DATA) -D $(INSTALLMMODE2) $(LANGSRC)/zh-CN/* -t $(DESTDIR)$(translationdir)/zh-CN )
	-( $(INSTALL_DATA) -D $(INSTALLMMODE2) $(LANGSRC)/zh-HK/* -t $(DESTDIR)$(translationdir)/zh-HK )
	-( $(INSTALL_DATA) -D $(INSTALLMMODE2) $(LANGSRC)/en-US/* -t $(DESTDIR)$(translationdir)/en-US )

.PHONY:clean
clean:
	-for libdir in ${LIB_DIR}; do \
		(cd $$libdir && make clean) ;\
	done

	cargo clean
	rm -rf Cargo.lock
