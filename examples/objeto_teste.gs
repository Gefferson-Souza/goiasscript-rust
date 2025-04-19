// Teste de objetos literais em GoiásScript
prosa("Testando objetos em GoiásScript");

// Objeto vazio
uai objeto_vazio é {};
prosa("Objeto vazio: " mais objeto_vazio);

// Objeto simples
uai pessoa é {
    nome: "João",
    idade: 30,
    ativo: certeza
};
prosa("Pessoa: " mais pessoa);

// Objeto com valores mistos
uai dados é {
    id: 1001,
    titulo: "Produto Teste",
    preco: 29.99,
    disponivel: certeza,
    categorias: ["eletrônicos", "promoção"]
};

prosa("Dados completos: " mais dados);

// Objeto com valores calculados
uai contador é 10;
uai config é {
    min: contador menos 5,
    max: contador mais 5,
    valor_padrao: contador
};

prosa("Configuração: " mais config);

prosa("Teste de objetos concluído!");