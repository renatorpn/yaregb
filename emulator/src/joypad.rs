pub struct Joypad{
    state: [u8; 8]
}

impl Joypad{

    pub fn new() -> Joypad{
        state: [1;8]
    }

    pub fn get_button_press(&self, button: usize){
        self.state[button]    
    }

    pub fn set_button_press(&mut self, button: usize){
        self.state[button] = 0;
    }

    pub fn reset_button_state(&mut self, button: usize){
        self.state[button] = 1;
    }
    
    pub fn get_button_mode(&mut self, mode:JoypadMode){
        JoypadMode::DIRECTION => {
            let up = self.state[UP_BUTTON];
            let down = self.state[DOWN_BUTTON];
            let left = self.state[LEFT_BUTTON];
            let right = self.state[RIGH_BUTTON];

            (down << 3) | (up << 2) | (left << 1) | right
        }
        JoypadMode::ACTION => {
            let start = self.state[START_BUTTON];
            let select = self.state[SELECT_BUTTON];
            let a = self.state[A_BUTTON];
            let b = self.state[B_BUTTON];

            (start << 3) | (select << 2) | (b << 1) | a
        }

    }
}