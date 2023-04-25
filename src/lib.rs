use rand::prelude::*;

pub struct Board{
    pub height: usize,
    pub width: usize,
    pub state: Vec<Vec<i32>> 
}

fn dead_state(width:usize, height:usize)->Vec<Vec<i32>>{
    vec![vec![0; width]; height]
}

impl Board{
    pub fn random_state(&mut self){
        let mut state = vec![vec![0; self.width]; self.height];
        let mut rng = rand::thread_rng();
        for i in 0..self.height{
            for j in 0..self.width{
                let random:f64 = rng.gen();
                if random>0.5{
                    state[i][j]=1;
                }else{
                    state[i][j]=0;
                }
            }
        }
        self.state = state;
    }
    pub fn render(&self){
            for i in 0..self.height{
                for j in 0..self.width{
                    if self.state[i][j]==0{
                        print!("   ");
                    }else{
                        print!("@");
                    }
                }
                println!();
            }
        }
    pub fn next_board_state(&mut self){
        let mut new_state = dead_state(self.width, self.height);
        for i in 0..self.height{
            for j in 0..self.width{
                if i==0{
                    if j==0{
                        let alive = self.state[i][j+1]+self.state[i+1][j]+self.state[i+1][j+1];
                        if self.state[i][j]==1 && (alive==0 || alive==1){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==1 && (alive==2 || alive==3){
                            new_state[i][j]=1;
                        } else if self.state[i][j]==1 && (alive>3){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==0 && alive==3{
                            new_state[i][j]=1;
                        }
                    } else if j==(self.width-1){
                        let alive = self.state[i][j-1]+self.state[i+1][j]+self.state[i+1][j-1];
                        if self.state[i][j]==1 && (alive==0 || alive==1){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==1 && (alive==2 || alive==3){
                            new_state[i][j]=1;
                        } else if self.state[i][j]==1 && (alive>3){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==0 && alive==3{
                            new_state[i][j]=1;
                        }
                    } else if j>0 && j<(self.width-1){
                        let alive = self.state[i][j-1]+self.state[i][j+1]+self.state[i+1][j-1]+self.state[i+1][j]+self.state[i+1][j+1];
                        if self.state[i][j]==1 && (alive==0 || alive==1){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==1 && (alive==2 || alive==3){
                            new_state[i][j]=1;
                        } else if self.state[i][j]==1 && (alive>3){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==0 && alive==3{
                            new_state[i][j]=1;
                        }
                    } 
                } else if i==(self.height-1){
                    if j==0{
                        let alive = self.state[i][j+1]+self.state[i-1][j]+self.state[i-1][j+1];
                        if self.state[i][j]==1 && (alive==0 || alive==1){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==1 && (alive==2 || alive==3){
                            new_state[i][j]=1;
                        } else if self.state[i][j]==1 && (alive>3){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==0 && alive==3{
                            new_state[i][j]=1;
                        }
                    } else if j==(self.width-1){
                        let alive = self.state[i][j-1]+self.state[i-1][j]+self.state[i-1][j-1];
                        if self.state[i][j]==1 && (alive==0 || alive==1){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==1 && (alive==2 || alive==3){
                            new_state[i][j]=1;
                        } else if self.state[i][j]==1 && (alive>3){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==0 && alive==3{
                            new_state[i][j]=1;
                        }
                    } else if j>0 && j<(self.width-1){
                        let alive = self.state[i][j-1]+self.state[i][j+1]+self.state[i-1][j-1]+self.state[i-1][j]+self.state[i-1][j+1];
                        if self.state[i][j]==1 && (alive==0 || alive==1){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==1 && (alive==2 || alive==3){
                            new_state[i][j]=1;
                        } else if self.state[i][j]==1 && (alive>3){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==0 && alive==3{
                            new_state[i][j]=1;
                        }
                    }
                } else if j==0{
                    if i>0 && i<(self.height-1){
                        let alive = self.state[i+1][j]+self.state[i-1][j]+self.state[i-1][j+1]+self.state[i+1][j+1]+self.state[i][j+1];
                        if self.state[i][j]==1 && (alive==0 || alive==1){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==1 && (alive==2 || alive==3){
                            new_state[i][j]=1;
                        } else if self.state[i][j]==1 && (alive>3){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==0 && alive==3{
                            new_state[i][j]=1;
                        }
                    }
                } else if j==(self.width-1){
                    if i>0 && i<(self.height-1){
                        let alive = self.state[i-1][j]+self.state[i+1][j]+self.state[i-1][j-1]+self.state[i][j-1]+self.state[i+1][j-1];
                        if self.state[i][j]==1 && (alive==0 || alive==1){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==1 && (alive==2 || alive==3){
                            new_state[i][j]=1;
                        } else if self.state[i][j]==1 && (alive>3){
                            new_state[i][j]=0;
                        } else if self.state[i][j]==0 && alive==3{
                            new_state[i][j]=1;
                        }
                    }
                } else {
                    let alive = self.state[i+1][j]+self.state[i-1][j]+self.state[i+1][j+1]+self.state[i+1][j-1]+self.state[i-1][j+1]+self.state[i-1][j-1]+self.state[i][j-1]+self.state[i][j+1];
                    if self.state[i][j]==1 && (alive==0 || alive==1){
                        new_state[i][j]=0;
                    } else if self.state[i][j]==1 && (alive==2 || alive==3){
                        new_state[i][j]=1;
                    } else if self.state[i][j]==1 && (alive>3){
                        new_state[i][j]=0;
                    } else if self.state[i][j]==0 && alive==3{
                        new_state[i][j]=1;
                    }
                }
            }
        }
        self.state = new_state;
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    fn setup1()->(Vec<Vec<i32>>, Vec<Vec<i32>>){
        let init_state = vec![vec![0;3]; 3];
        let expected_next_state = vec![vec![0; 3]; 3];
        (init_state, expected_next_state)
    }

    fn setup2()->(Vec<Vec<i32>>, Vec<Vec<i32>>){
        let init_state = vec![vec![0,0,1], vec![0,1,1], vec![0,0,0]];
        let expected_next_state = vec![vec![0,1,1], vec![0,1,1], vec![0,0,0]];
        (init_state, expected_next_state)
    }

    fn setup3()->(Vec<Vec<i32>>, Vec<Vec<i32>>){
        let init_state = vec![vec![1,1,1], vec![0,1,1], vec![0,0,0]];
        let expected_next_state = vec![vec![1,0,1], vec![1,0,1], vec![0,0,0]];
        (init_state, expected_next_state)
    }

    #[test]
    fn test1(){
        let (input, exp_output) = setup1();
        let mut board = Board{width:3, height:3, state:input};
        board.next_board_state();
        assert_eq!(exp_output, board.state);
    }

    #[test]
    fn test2(){
        let (input, exp_output) = setup2();
        let mut board = Board{width:3, height:3, state:input};
        board.next_board_state();
        assert_eq!(exp_output, board.state);
    }

    #[test]
    fn test3(){
        let (input, exp_output) = setup3();
        let mut board = Board{width:3, height:3, state:input};
        board.next_board_state();
        assert_eq!(exp_output, board.state);
    }
}
