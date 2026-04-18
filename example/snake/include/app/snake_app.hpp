#pragma once

#include "game_core/game_state.hpp"
#include "gameplay/game_logic.hpp"
#include "settings/game_settings.hpp"
#include "SDL3/SDL.h"

#include <cstdint>
#include <string>

namespace app {

class SnakeApp {
public:
    explicit SnakeApp(settings::GameSettings settings);
    ~SnakeApp();

    SnakeApp(const SnakeApp&) = delete;
    auto operator=(const SnakeApp&) -> SnakeApp& = delete;

    auto run() -> int;

private:
    auto initialize() -> bool;
    auto shutdown() -> void;
    auto process_events(bool& running) -> void;
    auto update_game(std::uint32_t now_ticks) -> void;
    auto render_frame() -> void;
    auto draw_grid(int cell_size) -> void;
    auto draw_snake(int cell_size) -> void;
    auto draw_food(int cell_size) -> void;
    auto update_window_title() -> void;

    settings::GameSettings settings_;
    game_core::GameState state_;
    gameplay::GameLogic logic_;

    SDL_Window* window_{nullptr};
    SDL_Renderer* renderer_{nullptr};
    std::uint32_t last_tick_ms_{0};
    bool initialized_{false};
    std::string mode_name_;
};

}  // namespace app
