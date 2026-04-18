#include "game_core/game_state.hpp"

#include "settings/game_settings.hpp"

#include <stdexcept>

namespace game_core {

GameState::GameState(const settings::GameSettings settings)
    : settings_(settings),
      cols_(settings::board_columns(settings.board)),
      rows_(settings::board_rows(settings.board)),
      rng_(settings.seed) {
    reset(settings.seed);
}

auto GameState::reset(const std::uint32_t seed) -> void {
    rng_.seed(seed);
    game_over_ = false;
    score_ = 0;
    direction_ = Direction::Right;

    const GridPos center{cols_ / 2, rows_ / 2};

    snake_.clear();
    snake_.push_back(center);
    snake_.push_back(GridPos{center.x - 1, center.y});
    snake_.push_back(GridPos{center.x - 2, center.y});

    food_ = random_free_cell();
}

auto GameState::cols() const -> int { return cols_; }

auto GameState::rows() const -> int { return rows_; }

auto GameState::snake() const -> const std::deque<GridPos>& { return snake_; }

auto GameState::food() const -> GridPos { return food_; }

auto GameState::direction() const -> Direction { return direction_; }

auto GameState::is_game_over() const -> bool { return game_over_; }

auto GameState::score() const -> int { return score_; }

auto GameState::settings() const -> const settings::GameSettings& { return settings_; }

auto GameState::set_direction(const Direction direction) -> void { direction_ = direction; }

auto GameState::set_food(const GridPos food) -> void { food_ = food; }

auto GameState::set_game_over(const bool is_over) -> void { game_over_ = is_over; }

auto GameState::add_score(const int score_delta) -> void { score_ += score_delta; }

auto GameState::push_head(const GridPos pos) -> void { snake_.push_front(pos); }

auto GameState::pop_tail() -> void { snake_.pop_back(); }

auto GameState::is_occupied(const GridPos pos) const -> bool {
    for (const auto& part : snake_) {
        if (part == pos) {
            return true;
        }
    }

    return false;
}

auto GameState::random_free_cell() -> GridPos {
    std::uniform_int_distribution<int> x_dist(0, cols_ - 1);
    std::uniform_int_distribution<int> y_dist(0, rows_ - 1);

    for (int i = 0; i < 4096; ++i) {
        const GridPos candidate{x_dist(rng_), y_dist(rng_)};
        if (!is_occupied(candidate)) {
            return candidate;
        }
    }

    for (int y = 0; y < rows_; ++y) {
        for (int x = 0; x < cols_; ++x) {
            const GridPos candidate{x, y};
            if (!is_occupied(candidate)) {
                return candidate;
            }
        }
    }

    throw std::runtime_error("No free cell available");
}

auto is_opposite(const Direction lhs, const Direction rhs) -> bool {
    return (lhs == Direction::Up && rhs == Direction::Down) ||
           (lhs == Direction::Down && rhs == Direction::Up) ||
           (lhs == Direction::Left && rhs == Direction::Right) ||
           (lhs == Direction::Right && rhs == Direction::Left);
}

auto next_cell(const GridPos head, const Direction direction) -> GridPos {
    GridPos result = head;

    switch (direction) {
        case Direction::Up:
            --result.y;
            break;
        case Direction::Down:
            ++result.y;
            break;
        case Direction::Left:
            --result.x;
            break;
        case Direction::Right:
            ++result.x;
            break;
    }

    return result;
}

}  // namespace game_core
