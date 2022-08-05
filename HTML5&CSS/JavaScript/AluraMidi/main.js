function tocaSom (seletorAudio) {
    
 const elemento = document.querySelector(seletorAudio);
    
    if (elemento && elemento.localName === 'audio') {
        
        elemento.play();
        
    }
    else {

        console.log('Elemento não encontrado ou seletor invalido');
        
    }

}

const ListaDeTeclas = document.querySelectorAll('.tecla');

let contador = 0;

//para
for (let contador = 0; contador < ListaDeTeclas.length; contador++) {

    const tecla = ListaDeTeclas[contador];
    const instrumento = tecla.classList[1];

 //template string
    const idAudio = `#som_${instrumento}`;

    tecla.onclick = function () {
        tocaSom(idAudio)
    };
    
    tecla.onkeydown = function (evento) {

        
        if (evento.code === 'Space' || evento.code === 'Enter') {
            tecla.classList.add('ativa');
        } 
        
    }
    
    tecla.onkeyup = function () {
        tecla.classList.remove('ativa')
    }
    
}
 