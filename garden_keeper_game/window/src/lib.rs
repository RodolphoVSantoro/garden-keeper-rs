use garden_keeper_types::{
    Position2D, RelativeWindowSize, Size2D, STANDARD_HEIGHT, STANDARD_WIDTH,
};

pub fn get_screen_size() -> Size2D {
    return Size2D {
        height: STANDARD_HEIGHT,
        width: STANDARD_WIDTH,
    };
    /*
    TODO: implement
    SDL_DisplayMode DM;
    if(SDL_GetDesktopDisplayMode(0, &DM)==0){
        *largura = DM.w;
        *altura = DM.h;
    }
    else {
        SDL_Log("SDL_GetDesktopDisplayMode failed: %s", SDL_GetError());
    }
    */
}

pub fn set_window_size(window_size: &mut Size2D, window_index: Option<u32>) {
    let index_window = window_index.unwrap_or(0);
    /*
    TODO: implement on wrapper
    SDL_Window* J = CGerenciadorJanelas::GetWindow(indjanela);
    SDL_SetWindowSize(J,largura,altura);
    */
}

pub fn get_window_size(window_index: Option<u32>) -> Size2D {
    let index_window = window_index.unwrap_or(0);
    return Size2D {
        height: 0,
        width: 0,
    };
    /*
    TODO: implement on wrapper
    SDL_Window* J = CGerenciadorJanelas::GetWindow(indjanela);
    SDL_GetWindowSize(J,largura,altura);
    */
}

pub fn outside_window(x: i32, y: i32, window_index: Option<u32>) -> bool {
    let index_window = window_index.unwrap_or(0);
    let mut dimensions = get_window_size(window_index);
    let outside_x = x < 0 || x >= dimensions.width;
    let outside_y = y < 0 || y >= dimensions.height;
    if (outside_x || outside_y) {
        return true;
    }
    return false;
}

pub fn show_FPS(id_font: Option<i32>, window_index: Option<u32>) {
    let id_font = id_font.unwrap_or(0);
    // let fps: [uchar;32];
    //TODO: implement on wrapper
    // sprintf(fps,"%.2f",GetFPS());
    let mut dimensions = get_window_size(window_index);
    // TODO: implement on wrapper
    // EscreverEsquerda(fps,largura/140,altura-altura/20,id_font);
}

//recebe numeros para a altura e largura, e multiplica dimensoes do objeto por esses dois numeros
pub fn resize_object(id: i32, multiply_dimensions: Size2D) {
    // TODO: implement both on wrapper
    // let object_size = GetDimensoesObjeto(id);
    // SetDimensoesObjeto(id,alturaObjeto*multiplyHeight,larguraObjeto*multiplyWidth);
}

pub fn get_centro_objeto(id: i32) -> Position2D {
    // TODO: implement both on wrapper
    // let object_size = GetDimensoesObjeto(id);
    // let position = GetXYObjeto(id);
    let mut position = Position2D { x: 0, y: 0 };
    let mut object_size = Size2D {
        height: 0,
        width: 0,
    };
    return Position2D {
        x: position.x + object_size.width / 2,
        y: position.y + object_size.height / 2,
    };
}
