NODE_BINDIR = ./front/node_modules/.bin
export PATH := $(NODE_BINDIR):$(PATH)
FRONT_TEMPLATE_POT ?= ./front/template.pot
FRONT_I18N_OUTPUT_DIR = ./front/src
FRONT_LOCALES = en_US fr_FR
FRONT_LOCALE_FILES ?= $(patsubst %,$(FRONT_I18N_OUTPUT_DIR)/locale/%/LC_MESSAGES/app.po,$(FRONT_LOCALES))
FRONT_GETTEXT_SOURCES ?= $(shell find ./front/src/ -name '*.jade' -o -name '*.html' -o -name '*.js' -o -name '*.vue' 2> /dev/null)

clean:
	rm -f $(FRONT_TEMPLATE_POT) $(FRONT_I18N_OUTPUT_DIR)/translations.json

makemessages: $(FRONT_TEMPLATE_POT)

translations: ./$(FRONT_I18N_OUTPUT_DIR)/translations.json

# Create a main .pot template, then generate .po files for each available language.
# Thanx to Systematic: https://github.com/Polyconseil/systematic/blob/866d5a/mk/main.mk#L167-L183
$(FRONT_TEMPLATE_POT): $(FRONT_GETTEXT_SOURCES)
# `dir` is a Makefile built-in expansion function which extracts the directory-part of `$@`.
# `$@` is a Makefile automatic variable: the file name of the target of the rule.
# => `mkdir -p /tmp/`
	mkdir -p $(dir $@)
# Extract gettext strings from templates files and create a POT dictionary template.
	gettext-extract --quiet --attribute v-translate --output $@ $(FRONT_GETTEXT_SOURCES)
# Generate .po files for each available language.
	@for lang in $(FRONT_LOCALES); do \
		export PO_FILE=$(FRONT_I18N_OUTPUT_DIR)/locale/$$lang/LC_MESSAGES/app.po; \
		mkdir -p $$(dirname $$PO_FILE); \
		if [ -f $$PO_FILE ]; then  \
			echo "msgmerge --update $$PO_FILE $@"; \
			msgmerge --lang=$$lang --update $$PO_FILE $@ || break ;\
		else \
			msginit --no-translator --locale=$$lang --input=$@ --output-file=$$PO_FILE || break ; \
			msgattrib --no-wrap --no-obsolete -o $$PO_FILE $$PO_FILE || break; \
		fi; \
	done;

$(FRONT_I18N_OUTPUT_DIR)/translations.json: $(FRONT_LOCALE_FILES)
	mkdir -p $(FRONT_I18N_OUTPUT_DIR)
	gettext-compile --output $@ $(FRONT_LOCALE_FILES)


services:
	docker-compose up -d
initdb: services
	diesel setup --config-file diesel-khnum.toml --migration-dir migrations/khnum/postgres/
migrate:
	diesel migration run --config-file diesel-khnum.toml --migration-dir migrations/khnum/postgres/
	diesel migration run --migration-dir migrations/postgres/
# sentry: 
# 	docker-compose -f sentry-docker-compose.yml up 
test:
	cargo +nightly-2019-11-29 test
	# cargo test
coverage:
	# launch tests & coverage, for tests only: "cargo test"
	cargo +nightly tarpaulin -v
run:
	# cargo watch -x run
	# cargo +nightly watch -x run
	cargo +nightly run
frontrun:
	cd front && yarn run serve
doc:
	cargo +nightly doc --open
