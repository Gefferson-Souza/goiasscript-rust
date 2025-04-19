// Teste de acesso a propriedades e elementos de arrays
prosa("Testando acesso a propriedades e elementos em GoiásScript");

// Objeto para teste
uai pessoa é {
    nome: "João",
    idade: 30,
    habilidades: ["programação", "música", "culinária"]
};

// Acesso a propriedades simples
prosa("Nome: " mais pessoa.nome);
prosa("Idade: " mais pessoa.idade);

// Acesso a arrays dentro de objetos
prosa("Primeira habilidade: " mais pessoa.habilidades[0]);
prosa("Segunda habilidade: " mais pessoa.habilidades[1]);
prosa("Terceira habilidade: " mais pessoa.habilidades[2]);

// Array para teste
uai numeros é [10, 20, 30, 40, 50];

// Acesso a elementos de array
prosa("Primeiro número: " mais numeros[0]);
prosa("Terceiro número: " mais numeros[2]);
prosa("Último número: " mais numeros[4]);

// Acesso a caracteres em string
uai texto é "GoiásScript";
prosa("Primeira letra: " mais texto[0]);
prosa("Quarta letra: " mais texto[3]);

// Acesso aninhado (objeto dentro de objeto)
uai empresa é {
    nome: "TechGoiás",
    endereco: {
        cidade: "Goiânia",
        estado: "Goiás"
    },
    funcionarios: [
        { nome: "Maria", cargo: "Desenvolvedora" },
        { nome: "Pedro", cargo: "Designer" }
    ]
};

prosa("Empresa: " mais empresa.nome);
prosa("Cidade: " mais empresa.endereco.cidade);
prosa("Estado: " mais empresa.endereco.estado);
prosa("Primeiro funcionário: " mais empresa.funcionarios[0].nome);
prosa("Cargo: " mais empresa.funcionarios[0].cargo);

prosa("Teste concluído!");