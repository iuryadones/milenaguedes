.PHONY: setup build run check test clean deploy help

TRUNK := trunk
TARGET := wasm32-unknown-unknown
PUBLIC_URL := /

setup:
	cargo install trunk --locked
	rustup target add $(TARGET)
	@echo "✅ Setup completo. Execute 'make run' para iniciar."

build:
	$(TRUNK) build --release --public-url $(PUBLIC_URL)
	@echo "✅ Build gerado em dist/"

run:
	$(TRUNK) serve

check:
	cargo check --target $(TARGET)

test: check

clean:
	cargo clean
	rm -rf dist
	@echo "✅ Projeto limpo."

deploy: build
	cp dist/index.html dist/404.html
	echo "/* /index.html 200" > dist/_redirects
	echo "milenaguedes.com" > dist/CNAME
	@echo ""
	@echo "═══════════════════════════════════════════════════"
	@echo "  Deploy pronto! Copie o conteúdo de dist/ para a"
	@echo "  branch gh-pages:"
	@echo ""
	@echo "    cp -r dist/* /caminho/para/gh-pages/"
	@echo "    cd /caminho/para/gh-pages/"
	@echo "    git add . && git commit -m \"deploy\""
	@echo "    git push origin gh-pages"
	@echo "═══════════════════════════════════════════════════"

help:
	@echo ""
	@echo "📋 Comandos disponíveis"
	@echo "═══════════════════════════════════════════════════"
	@echo "  make setup   — Instala trunk + target wasm32"
	@echo "  make build   — Build de produção (gera dist/)"
	@echo "  make run     — Servidor de dev em localhost:8080"
	@echo "  make check   — Verifica compilação (wasm)"
	@echo "  make test    — Alias para check"
	@echo "  make clean   — Remove build + cache"
	@echo "  make deploy  — Build + gera _redirects + CNAME"
	@echo "  make help    — Mostra esta mensagem"
	@echo "═══════════════════════════════════════════════════"
