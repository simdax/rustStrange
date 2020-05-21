#include <iostream>

struct State {
    double phase;
    double freq;
    double inc;
    float result;
};

extern "C" {
    State init(unsigned sr);
}

int main(){
    auto state = init(48000);
    std::cout << "fsdafd " << state.freq << " " << state.phase;
}
