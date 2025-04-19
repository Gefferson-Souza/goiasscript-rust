// Teste de classes e OOP em GoiásScript
prosa("Testando classes em GoiásScript");

// Definição de uma classe Animal básica
arruma_trem Animal {
    // Construtor da classe
    aprepara_trem(nome, idade) {
        uai ocê.nome é nome;
        uai ocê.idade é idade;
        uai ocê.tipo é "animal";
    }
    
    // Método para obter descrição
    descricao() {
        faz_favor ocê.nome mais " (" mais ocê.tipo mais ", " mais ocê.idade mais " anos)";
    }
    
    // Método para fazer som
    faz_som() {
        prosa(ocê.nome mais " faz algum som");
    }
}

// Classe filha que herda de Animal
arruma_trem Cachorro inherda_de Animal {
    // Construtor da classe filha
    aprepara_trem(nome, idade, raca) {
        // Chama construtor da classe pai
        uai ocê.nome é nome;
        uai ocê.idade é idade;
        uai ocê.tipo é "cachorro";
        uai ocê.raca é raca;
    }
    
    // Sobrescrevendo o método faz_som
    faz_som() {
        prosa(ocê.nome mais " faz Au Au!");
    }
    
    // Método específico da classe filha
    abana_rabo() {
        prosa(ocê.nome mais " está abanando o rabo!");
    }
}

// Teste da classe Animal
prosa("Criando um animal genérico...");
uai animal é faz_um Animal("Totó", 3);
prosa("Descrição: " mais animal.descricao());
animal.faz_som();

// Teste da classe Cachorro
prosa("Criando um cachorro...");
uai cachorro é faz_um Cachorro("Rex", 2, "Vira-lata");
prosa("Descrição: " mais cachorro.descricao());
cachorro.faz_som(); // Método sobrescrito
cachorro.abana_rabo(); // Método específico

prosa("Teste de classes concluído!");