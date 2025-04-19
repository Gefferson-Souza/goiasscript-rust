# GoiásScript-Rust: O Compilador Goiano em Rust

**Versão:** 0.1.0 (em desenvolvimento)
**Data:** 2025-04-19
**Autor:** Gefferson-Souza

---

## Sumário

1.  [Introdução](#1-introdução)
2.  [Por que Rust?](#2-por-que-rust)
3.  [Instalação e Uso](#3-instalação-e-uso)
4.  [Sintaxe Básica (Atual)](#4-sintaxe-básica-atual)
5.  [Funcionalidades Planejadas](#5-funcionalidades-planejadas)
    *   Classes e Objetos
    *   Programação Assíncrona (Async/Await)
    *   Estruturas de Dados (Arrays, Maps, Sets)
    *   Tratamento de Erros Avançado
6.  [Exemplos](#6-exemplos)
7.  [Referência de Palavras-chave (Atual)](#7-referência-de-palavras-chave-atual)
8.  [Como Contribuir](#8-como-contribuir)
9.  [Licença](#9-licença)

---

## 1. Introdução

GoiásScript é uma linguagem de programação baseada no dialeto goiano do interior do Brasil. Foi projetada para ser uma forma divertida e culturalmente relevante de aprender e praticar programação, especialmente para pessoas da região Centro-Oeste do Brasil.

Esta é a implementação do compilador em **Rust**, que traduz o código GoiásScript (`.gs`) para código Rust (`.rs`) e, opcionalmente, compila este último em um executável binário. O objetivo é alavancar a performance, segurança e o robusto sistema de tipos do Rust.

## 2. Por que Rust?

A escolha do Rust para esta versão do compilador visa trazer:

*   **Performance:** Código compilado nativamente tende a ser mais rápido que código interpretado ou JIT-compilado (como JavaScript).
*   **Segurança de Memória:** Rust previne erros comuns como null pointer dereferences e data races em tempo de compilação.
*   **Sistema de Tipos Forte:** Ajuda a detectar erros mais cedo no desenvolvimento.
*   **Ferramental:** Cargo (gerenciador de pacotes e build system), rustfmt (formatador), clippy (linter) oferecem uma ótima experiência de desenvolvimento.
*   **Concorrência e Async:** O ecossistema Rust possui suporte de primeira classe para programação concorrente e assíncrona segura e eficiente.

## 3. Instalação e Uso

### 3.1 Pré-requisitos

*   **Rust Toolchain:** Inclui `rustc` (compilador) e `cargo` (gerenciador de pacotes). Instale via [rustup.rs](https://rustup.rs/).

### 3.2 Compilando o Compilador GoiásScript

```bash
# 1. Clone o repositório
git clone https://github.com/Gefferson-Souza/goiasscript-rust.git

# 2. Entre no diretório
cd goiasscript-rust

# 3. Compile o projeto (o executável estará em target/release/goiasscript)
cargo build --release
```

### 3.3 Usando o Compilador

O compilador `goiasscript` aceita um arquivo `.gs` como entrada e possui diferentes modos de operação:

```bash
# Sintaxe básica:
# ./target/release/goiasscript [OPÇÕES] <ARQUIVO_ENTRADA.gs>

# Exemplo 1: Compilar 'ola.gs' para um executável (modo padrão: build)
./target/release/goiasscript examples/ola.gs

# Exemplo 2: Compilar 'ola.gs' em modo release (otimizado)
./target/release/goiasscript examples/ola.gs --mode release

# Exemplo 3: Apenas traduzir 'ola.gs' para 'ola.rs' (sem compilar)
./target/release/goiasscript examples/ola.gs --mode translate

# Exemplo 4: Compilar e mostrar o código Rust gerado no terminal
./target/release/goiasscript examples/ola.gs --show-code

# Exemplo 5: Especificar um nome para o arquivo Rust de saída
./target/release/goiasscript examples/ola.gs --output meu_codigo.rs --mode translate
```

## 4. Sintaxe Básica (Atual)

O compilador Rust atualmente suporta um subconjunto da linguagem GoiásScript original:

*   **Declaração de Variáveis:**
    *   Constantes: `uai nome é "Valor";` (gera `let nome = "Valor";`)
    *   Mutáveis: `trem contador é 0;` (gera `let mut contador = 0;`)
*   **Atribuição:** `contador é contador mais 1;` (gera `contador = contador + 1;`)
*   **Saída no Console:** `prosa("Mensagem");` (gera `println!("Mensagem");`)
    *   Suporta concatenação básica: `prosa("Valor: " + contador);` (gera `println!("Valor: {}", contador);`)
*   **Funções:**
    *   Declaração: `presta_serviço minhaFuncao(param1, param2) { ... }` (gera `fn minhaFuncao(param1: impl std::fmt::Display, param2: impl std::fmt::Display) { ... }`)
    *   Função Principal: `presta_serviço principal() { ... }` (gera `fn main() { ... }`)
    *   Retorno: `faz_favor valor;` (gera `return valor;`)
*   **Estruturas de Controle:**
    *   `se_ocê_quiser (condicao) { ... }` (gera `if condicao { ... }`)
    *   `se_ocê_quiser (condicao) { ... } se_não { ... }` (gera `if condicao { ... } else { ... }`)
    *   `enquanto_tiver (condicao) { ... }` (gera `while condicao { ... }`)
    *   `vai_indo (inicializador; condicao; incremento) { ... }` (gera `for ...`) - *Suporte básico*
    *   `para_com_isso;` (gera `break;`) - *Ainda não implementado no tradutor*
    *   `continua_aí;` (gera `continue;`) - *Ainda não implementado no tradutor*
*   **Operadores:**
    *   Aritméticos: `mais`, `menos`, `vezes`, `dividido`, `sobrou`
    *   Comparação: `é_igualim`, `diferente`, `maior_que`, `menor_que`, `pelo_menos`, `no_máximo`
    *   Lógicos: `e_mais`, `ou_então`
*   **Literais:**
    *   Strings: `"texto"`
    *   Números: `10`, `3.14` (traduzidos para `f64`)
    *   Booleanos: `certeza` (true), `de_jeito_nenhum` (false)
    *   Nulo: `vazio` (traduzido para `None`, requer contexto de `Option`) - *Tradução pode precisar de refinamento*
*   **Comentários:** `// Comentário de linha`, `/* Comentário de bloco */`

## 5. Funcionalidades Planejadas

Esta versão em Rust está em desenvolvimento. As seguintes funcionalidades da GoiásScript original (JS) são planejadas para futuras versões:

*   **Classes e Objetos:** Suporte completo para `arruma_trem`, `aprepara_trem`, `ocê`, `inherda_de`, `num_muda`, `é_tipo_de`, `faz_um`.
*   **Programação Assíncrona:** Implementação robusta de `vai_na_frente`, `espera_um_cadim` utilizando um runtime async do Rust (e.g., Tokio), e mapeamento de `promessa`.
*   **Estruturas de Dados:**
    *   Suporte a literais de Array (`[...]`).
    *   Suporte a literais de Objeto/Map (`{chave: valor, ...}`).
    *   Implementação de `Map` e `Set` (`faz_um Map()`, `faz_um Set()`).
    *   Acesso a elementos e propriedades (`array[indice]`, `objeto.propriedade`).
*   **Tratamento de Erros Avançado:** Implementação de `vixe` (panic/throw), `por_fim` (finally), e melhor tradução de `tenta_aí`/`se_der_ruim` para o `Result` do Rust.
*   **Loops Avançados:** `for...em` e `for...de`.
*   **Funções:** Arrow functions (`=>`), parâmetros com valores padrão.
*   **Sistema de Tipos:** Refinamento da tradução de tipos ou introdução de um tipo dinâmico (`GoiasValue`) para melhor interoperabilidade.

## 6. Exemplos

```goiasscript
// examples/ola.gs
presta_serviço principal() {
  prosa("Uai, GoiásScript em Rust!");
}

// Compilar e rodar:
// ./target/release/goiasscript examples/ola.gs && ./examples/ola
```

```goiasscript
// examples/simples.gs
presta_serviço principal() {
    uai nome é "Goiás";
    trem contador é 0;

    prosa("Bem-vindo ao " + nome + "Script!");

    enquanto_tiver (contador menor_que 3) {
        contador é contador mais 1;
        prosa("Contagem: " + contador);
    }

    se_ocê_quiser (contador é_igualim 3) {
        prosa("Chegou no três!");
    } se_não {
        prosa("Num chegou no três..."); // Isso não deve acontecer neste exemplo
    }
}

// Compilar e rodar:
// ./target/release/goiasscript examples/simples.gs && ./examples/simples
```

## 7. Referência de Palavras-chave (Atual)

| GoiásScript        | Tradução Rust (Aproximada) | Status Atual      |
|--------------------|----------------------------|-------------------|
| `uai`              | `let`                      | ✅ Implementado   |
| `trem`             | `let mut`                  | ✅ Implementado   |
| `é`                | `=`                        | ✅ Implementado   |
| `presta_serviço`   | `fn`                       | ✅ Implementado   |
| `faz_favor`        | `return`                   | ✅ Implementado   |
| `se_ocê_quiser`    | `if`                       | ✅ Implementado   |
| `se_num_for`       | `else if`                  | ❌ Não implementado |
| `se_não`           | `else`                     | ✅ Implementado   |
| `vai_indo`         | `for` / `loop`             | ✅ Básico         |
| `enquanto_tiver`   | `while`                    | ✅ Implementado   |
| `para_com_isso`    | `break`                    | ⚠️ Parser OK, Tradutor NÃO |
| `continua_aí`      | `continue`                 | ⚠️ Parser OK, Tradutor NÃO |
| `em`               | (iteration)                | ❌ Não implementado |
| `de`               | (iteration)                | ❌ Não implementado |
| `ocê`              | `self`                     | ❌ Não implementado |
| `prosa`            | `println!`                 | ✅ Implementado   |
| `reclama`          | `eprintln!`                | ⚠️ Lexer OK, Parser/Tradutor NÃO |
| `vai_na_frente`    | `async`                    | ⚠️ Parser OK, Tradutor Básico |
| `espera_um_cadim`  | `await`                    | ❌ Não implementado |
| `tenta_aí`         | `match` / `try` block      | ⚠️ Parser OK, Tradutor NÃO |
| `se_der_ruim`      | `Err(e) =>` / `catch`      | ⚠️ Parser OK, Tradutor NÃO |
| `por_fim`          | `finally`                  | ❌ Não implementado |
| `e_mais`           | `&&`                       | ✅ Implementado   |
| `ou_então`         | `||`                       | ✅ Implementado   |
| `num_é`            | `!`                        | ❌ Não implementado |
| `faz_um`           | `Struct::new()` / `new`    | ❌ Não implementado |
| `vixe`             | `panic!`                   | ❌ Não implementado |
| `arruma_trem`      | `struct` / `class`         | ❌ Não implementado |
| `aprepara_trem`    | `fn new()` / `constructor` | ❌ Não implementado |
| `inherda_de`       | `impl Trait for` / `extends`| ❌ Não implementado |
| `num_muda`         | `static` / `const`         | ❌ Não implementado |
| `é_tipo_de`        | (type check) / `instanceof`| ❌ Não implementado |
| `é_igualim`     | `==`                       | ✅ Implementado   |
| `diferente`     | `!=`                       | ✅ Implementado   |
| `maior_que`     | `>`                        | ✅ Implementado   |
| `menor_que`     | `<`                        | ✅ Implementado   |
| `pelo_menos`    | `>=`                       | ✅ Implementado   |
| `no_máximo`     | `<=`                       | ✅ Implementado   |
| `mais`       | `+`                        | ✅ Implementado   |
| `menos`      | `-`                        | ✅ Implementado   |
| `vezes`      | `*`                        | ✅ Implementado   |
| `dividido`   | `/`                        | ✅ Implementado   |
| `sobrou`     | `%`                        | ✅ Implementado   |
| `certeza`          | `true`                     | ✅ Implementado   |
| `de_jeito_nenhum`  | `false`                    | ✅ Implementado   |
| `vazio`            | `None`                     | ✅ Implementado (Tradução pode variar) |
| `sei_lá`           | `None` / `undefined`       | ⚠️ Lexer OK, Parser/Tradutor NÃO |

*(✅ Implementado | ⚠️ Parcial/Básico | ❌ Não implementado)*

## 8. Como Contribuir

Contribuições são bem-vindas! Siga estes passos:

1.  Faça um **Fork** do repositório.
2.  Crie uma nova **Branch** para sua feature ou correção (`git checkout -b minha-feature`).
3.  Implemente suas mudanças e adicione testes se aplicável.
4.  Faça **Commit** das suas alterações (`git commit -m "Adiciona funcionalidade X"`).
5.  Faça **Push** para a sua branch (`git push origin minha-feature`).
6.  Abra um **Pull Request** no repositório original.

Por favor, mantenha a consistência do código e adicione documentação relevante.

## 9. Licença

Este projeto está licenciado sob a **MIT License**. Veja o arquivo `LICENSE` (se existir) para mais detalhes.

---

*"Programar em Rust é bão demais da conta, sô!"*
