#include "app/snake_app.hpp"

#include "SDL3/SDL.h"

#include <algorithm>
#include <cstddef>
#include <sstream>

namespace {

struct Theme {
    SDL_Color bg;
    SDL_Color grid;
    SDL_Color snake;
    SDL_Color snake_head;
    SDL_Color food;
};

auto make_theme(const settings::RenderMode mode) -> Theme {
    switch (mode) {
        case settings::RenderMode::Pixel:
            return Theme{
                SDL_Color{22, 33, 20, 255},
                SDL_Color{35, 52, 32, 255},
                SDL_Color{114, 190, 90, 255},
                SDL_Color{190, 242, 100, 255},
                SDL_Color{218, 76, 87, 255},
            };
        case settings::RenderMode::Retro:
            return Theme{
                SDL_Color{36, 27, 18, 255},
                SDL_Color{62, 42, 24, 255},
                SDL_Color{239, 170, 0, 255},
                SDL_Color{255, 212, 84, 255},
                SDL_Color{239, 91, 36, 255},
            };
        case settings::RenderMode::Neon:
            return Theme{
                SDL_Color{7, 13, 24, 255},
                SDL_Color{17, 30, 54, 255},
                SDL_Color{32, 210, 187, 255},
                SDL_Color{118, 255, 240, 255},
                SDL_Color{255, 62, 133, 255},
            };
    }

    return Theme{};
}

auto set_color(SDL_Renderer* renderer, const SDL_Color color) -> void {
    SDL_SetRenderDrawColor(renderer, color.r, color.g, color.b, color.a);
}

}  // namespace

namespace app {

SnakeApp::SnakeApp(const settings::GameSettings settings)
    : settings_(settings),
      state_(settings_),
      mode_name_(settings::render_mode_name(settings.render)) {}

SnakeApp::~SnakeApp() { shutdown(); }

auto SnakeApp::run() -> int {
    if (!initialize()) {
        return 1;
    }

    bool running = true;
    last_tick_ms_ = static_cast<std::uint32_t>(SDL_GetTicks());

    while (running) {
        process_events(running);

        const auto now = static_cast<std::uint32_t>(SDL_GetTicks());
        update_game(now);
        render_frame();

        SDL_Delay(1);
    }

    return 0;
}

auto SnakeApp::initialize() -> bool {
    if (initialized_) {
        return true;
    }

    if (!SDL_Init(SDL_INIT_VIDEO)) {
        SDL_Log("SDL_Init failed: %s", SDL_GetError());
        return false;
    }

    constexpr int kCellSize = 28;
    const int width = state_.cols() * kCellSize;
    const int height = state_.rows() * kCellSize;

    window_ = SDL_CreateWindow("SnakeSDL3", width, height, SDL_WINDOW_RESIZABLE);
    if (window_ == nullptr) {
        SDL_Log("SDL_CreateWindow failed: %s", SDL_GetError());
        shutdown();
        return false;
    }

    renderer_ = SDL_CreateRenderer(window_, nullptr);
    if (renderer_ == nullptr) {
        SDL_Log("SDL_CreateRenderer failed: %s", SDL_GetError());
        shutdown();
        return false;
    }

    initialized_ = true;
    update_window_title();
    return true;
}

auto SnakeApp::shutdown() -> void {
    if (renderer_ != nullptr) {
        SDL_DestroyRenderer(renderer_);
        renderer_ = nullptr;
    }

    if (window_ != nullptr) {
        SDL_DestroyWindow(window_);
        window_ = nullptr;
    }

    if (initialized_) {
        SDL_Quit();
        initialized_ = false;
    }
}

auto SnakeApp::process_events(bool& running) -> void {
    SDL_Event event;
    while (SDL_PollEvent(&event)) {
        switch (event.type) {
            case SDL_EVENT_QUIT:
                running = false;
                break;

            case SDL_EVENT_KEY_DOWN: {
                switch (event.key.key) {
                    case SDLK_ESCAPE:
                        running = false;
                        break;
                    case SDLK_UP:
                    case SDLK_W:
                        logic_.request_turn(game_core::Direction::Up);
                        break;
                    case SDLK_DOWN:
                    case SDLK_S:
                        logic_.request_turn(game_core::Direction::Down);
                        break;
                    case SDLK_LEFT:
                    case SDLK_A:
                        logic_.request_turn(game_core::Direction::Left);
                        break;
                    case SDLK_RIGHT:
                    case SDLK_D:
                        logic_.request_turn(game_core::Direction::Right);
                        break;
                    case SDLK_R:
                        state_.reset(settings_.seed);
                        break;
                    default:
                        break;
                }
                break;
            }

            default:
                break;
        }
    }
}

auto SnakeApp::update_game(const std::uint32_t now_ticks) -> void {
    const int tick_ms = settings::tick_interval_ms(settings_.difficulty);
    if (now_ticks - last_tick_ms_ < static_cast<std::uint32_t>(tick_ms)) {
        return;
    }

    last_tick_ms_ = now_ticks;
    logic_.tick(state_);
    update_window_title();
}

auto SnakeApp::render_frame() -> void {
    const Theme theme = make_theme(settings_.render);
    const int cell_size = std::max(8, 760 / std::max(1, state_.cols()));

    set_color(renderer_, theme.bg);
    SDL_RenderClear(renderer_);

    if (settings_.show_grid) {
        draw_grid(cell_size);
    }

    draw_food(cell_size);
    draw_snake(cell_size);

    SDL_RenderPresent(renderer_);
}

auto SnakeApp::draw_grid(const int cell_size) -> void {
    const Theme theme = make_theme(settings_.render);
    set_color(renderer_, theme.grid);

    for (int y = 0; y < state_.rows(); ++y) {
        for (int x = 0; x < state_.cols(); ++x) {
            SDL_FRect rect{
                static_cast<float>(x * cell_size),
                static_cast<float>(y * cell_size),
                static_cast<float>(cell_size - 1),
                static_cast<float>(cell_size - 1),
            };
            SDL_RenderRect(renderer_, &rect);
        }
    }
}

auto SnakeApp::draw_snake(const int cell_size) -> void {
    const Theme theme = make_theme(settings_.render);
    const auto& snake = state_.snake();

    for (std::size_t i = 0; i < snake.size(); ++i) {
        const auto& part = snake[i];
        set_color(renderer_, i == 0 ? theme.snake_head : theme.snake);

        SDL_FRect rect{
            static_cast<float>(part.x * cell_size + 1),
            static_cast<float>(part.y * cell_size + 1),
            static_cast<float>(cell_size - 2),
            static_cast<float>(cell_size - 2),
        };

        SDL_RenderFillRect(renderer_, &rect);
    }
}

auto SnakeApp::draw_food(const int cell_size) -> void {
    const Theme theme = make_theme(settings_.render);
    set_color(renderer_, theme.food);

    const auto food = state_.food();
    SDL_FRect rect{
        static_cast<float>(food.x * cell_size + 3),
        static_cast<float>(food.y * cell_size + 3),
        static_cast<float>(cell_size - 6),
        static_cast<float>(cell_size - 6),
    };

    SDL_RenderFillRect(renderer_, &rect);
}

auto SnakeApp::update_window_title() -> void {
    std::ostringstream builder;
    builder << "SnakeSDL3"
            << " | Score: " << state_.score()
            << " | Mode: " << mode_name_;

    if (state_.is_game_over()) {
        builder << " | Game Over (R to restart)";
    }

    SDL_SetWindowTitle(window_, builder.str().c_str());
}

}  // namespace app
