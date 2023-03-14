const STARTINGPOINTS: i32 = 300;
const STARTINGROUND: i32 = 1;

use garden_keeper_game::{CreateGame, STATIC_COMPONENTS};

//TODO: object to handle events and keyboard

pub fn main() {
    CreateGame("Meu Jogo");

    garden_keeper_game::GameStaticComponents::setDimensions(800, 600);
    
    let player: i32 = 1;
    let px: i32 = 0;
    let py: i32 = 0;
    let player_largura: i32 = 0;
    let player_altura: i32 = 0;
    let hp_player: i32 = 2;

    Cena *C=NULL;
    tipoMagia tM;
    int essencia[NMAGIAS];
    bool comprados[NMAGIAS];
    int pontos, pontosGastos;
    int nRound, toSpawn, spawned;
    double tempoEspera,tCooldown;
    int timerTeclado=-1,cooldown=-1,timerInimigos=-1,timerEndRound=-1,timerPAUSE=-1;
    idFonteHUD=0;
    int timerLimFPS=CriaTimer();
    Botao *botaoRetry,*botaoSaida;

    carregaJogo(player,p_largura,p_altura,&botaoRetry,&botaoSaida);
    criaTimersJogo(timerTeclado,cooldown,timerInimigos,timerEndRound,timerPAUSE);
    setupPartida(&C,player,px,py,pontos,pontosGastos,nRound,toSpawn,spawned,tempoEspera,hpPlayer,comprados,essencia,tM,tCooldown,STARTINGROUND);
    meuTeclado = GetTeclado();
    estadoJogo=MIDROUND;

    while(JogoRodando() && estadoJogo!=FIM){
        if(TempoDecorrido(timerLimFPS)>0.01){
            ReiniciaTimer(timerLimFPS);
        }
        else continue;
        evento = GetEvento();
        pressOrReleaseMouse(evento);
        IniciaDesenho();
        if(estadoJogo==MIDROUND || estadoJogo==ENDROUND){
            //Movimentacao
            if(TempoDecorrido(timerTeclado)>playerSpeedCooldown){
                bool moveu = movimentacaoPlayer(C, px, py, p_largura, p_altura,meuTeclado);
                if(moveu)
                    ReiniciaTimer(timerTeclado);
            }
            //Magias
            trocaMagia(C,tM,comprados,meuTeclado,evento);
            if(TempoDecorrido(cooldown)>tCooldown){
                tCooldown=0.0;
                bool jogou = jogaMagia(C,px,py,p_largura,p_altura,evento,meuTeclado,essencia,tCooldown,tM);
                if(jogou){
                    ReiniciaTimer(cooldown);
                }
            }

            //PAUSE
            if(meuTeclado[TECLA_ESC] && TempoDecorrido(timerPAUSE)>0.2){
                ReiniciaTimer(timerPAUSE);
                estadoAnterior=estadoJogo;
                estadoJogo=PAUSE;
            }
        }
        if(estadoJogo==MIDROUND){
            //inimigos
            if(TempoDecorrido(timerInimigos)>3.0/((nRound)*log((double)nRound+1.0)) && vivos<50 && toSpawn>0){
                criaFormigas(C,maxHPFormiga);
                ReiniciaTimer(timerInimigos);
                criados++;
                vivos++;
                toSpawn--;
                spawned++;
            }
            if(toSpawn<=0 && vivos==0){
                toSpawn=calculaQTDInimigos(nRound+1);
                maxHPFormiga=calculaHPInimigos(nRound+1);
                estadoJogo=ENDROUND;
                ReiniciaTimer(timerEndRound);
            }
        }

        DesenhaCena(C,estadoJogo!=PAUSE);
        pontos=(criados-vivos)*60+pontosDeHits-pontosGastos+STARTINGPOINTS;
        escreveNumero(pontos,larJanela/27,altJanela/4,idFonteHUD);
        escreveNumero(nRound,larJanela/27,altJanela/8,idFonteHUD);
        escreveNumero(C->F->hpPlayer,larJanela/2,altJanela/15,idFonteHUD);
        EscreveFPS(idFonteHUD);

        if(estadoJogo==ENDROUND){
            double t = TempoDecorrido(timerEndRound);
            float L;
            if(crescente)L=LUZFUNDO+(20*t/tempoEspera);
            else L=LUZFUNDO-(20*t/tempoEspera);
            if(L==200)crescente=false;
            else if(L==20)crescente=true;
            setLuzFundo(L);
            escreveNumero(t,larJanela/2,altJanela-altJanela/10,idFonteHUD);
            if(t>tempoEspera){
                if(LUZFUNDO==200)crescente=false;
                else if(LUZFUNDO==20)crescente=true;
                if(crescente)LUZFUNDO+=20;
                else LUZFUNDO-=20;
                setLuzFundo(LUZFUNDO);
                nRound++;
                estadoJogo=MIDROUND;
            }
        }

        if(estadoJogo==PAUSE){
            telaPause(&botaoRetry, &botaoSaida, evento);
            PausaTimer(timerEndRound);
            if(meuTeclado[TECLA_ESC] && TempoDecorrido(timerPAUSE)>0.2){
                DespausaTimer(timerEndRound);
                estadoJogo=estadoAnterior;
                estadoAnterior=PAUSE;
                ReiniciaTimer(C->F->timerAT);
                ReiniciaTimer(timerPAUSE);
            }
        }

        if(estadoJogo==PERDEU){
            telaDerrota(criados-vivos,&botaoRetry,&botaoSaida,evento);
        }
        else{
            telaEssencia(C,tM,essencia);
        }

        if(estadoJogo==MIDROUND || estadoJogo==ENDROUND){
            menuCompra(pontos,pontosGastos,comprados,essencia,evento,meuTeclado);
        }

        if(estadoJogo==RETRY){
            setupPartida(&C,player,px,py,pontos,pontosGastos,nRound,toSpawn,spawned,tempoEspera,hpPlayer,comprados,essencia,tM,tCooldown,STARTINGROUND);
            estadoJogo=MIDROUND;
        }

        EncerraDesenho();
    }

    liberaCena(&C);
    FinalizaJogo();

    return 0;
}
