.PHONY: setup build run check test clean deploy deploy-gh-pages help

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
	@echo "  branch gh-pages (apenas build na raiz, sem site/):"
	@echo ""
	@echo "    cp -r dist/* /caminho/para/gh-pages/"
	@echo "    cd /caminho/para/gh-pages/"
	@echo "    rm -rf site/  # mantém apenas build na raiz"
	@echo "    git add . && git commit -m \"deploy\""
	@echo "    git push origin gh-pages"
	@echo "═══════════════════════════════════════════════════"

deploy-gh-pages: deploy
	git add -A && git commit -m "deploy: deploy $$(date +%Y-%m-%d)" || true
	git push origin master
	rm -rf /tmp/gh-deploy
	git clone --no-checkout file://$(CURDIR) /tmp/gh-deploy
	cd /tmp/gh-deploy && git checkout --orphan gh-pages && cp -r $(CURDIR)/dist/* . && rm -rf dist && git add -A && git commit -m "deploy $$(date +%Y-%m-%d)" && git push origin gh-pages --force
	rm -rf /tmp/gh-deploy
	@echo "✅ Deploy completo em gh-pages"

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
	@echo "  make deploy-gh-pages  — Deploy completo: build + commit master + gh-pages limpo"
	@echo "  make help    — Mostra esta mensagem"
	@echo "═══════════════════════════════════════════════════"
