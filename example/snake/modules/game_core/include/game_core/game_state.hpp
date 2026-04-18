#pragma once

#include "settings/game_settings.hpp"

#include <cstdint>
#include <deque>
#include <random>

namespace game_core {

// Integer coordinate on the logical board grid.
struct GridPos {
    int x{0};
    int y{0};

    [[nodiscard]] auto operator==(const GridPos& other) const -> bool {
        return x == other.x && y == other.y;
    }
};

enum class Direction : std::uint8_t {
    Up,
    Down,
    Left,
    Right
};

// Stores complete mutable game state.
class GameState {
public:
    explicit GameState(settings::GameSettings settings);

    auto reset(std::uint32_t seed) -> void;

    [[nodiscard]] auto cols() const -> int;
    [[nodiscard]] auto rows() const -> int;
    [[nodiscard]] auto snake() const -> const std::deque<GridPos>&;
    [[nodiscard]] auto food() const -> GridPos;
    [[nodiscard]] auto direction() const -> Direction;
    [[nodiscard]] auto is_game_over() const -> bool;
    [[nodiscard]] auto score() const -> int;
    [[nodiscard]] auto settings() const -> const settings::GameSettings&;

    auto set_direction(Direction direction) -> void;
    auto set_food(GridPos food) -> void;
    auto set_game_over(bool is_over) -> void;
    auto add_score(int score_delta) -> void;
    auto push_head(GridPos pos) -> void;
    auto pop_tail() -> void;

    [[nodiscard]] auto is_occupied(GridPos pos) const -> bool;
    [[nodiscard]] auto random_free_cell() -> GridPos;

private:
    settings::GameSettings settings_;
    int cols_{0};
    int rows_{0};
    std::deque<GridPos> snake_;
    GridPos food_{};
    Direction direction_{Direction::Right};
    bool game_over_{false};
    int score_{0};
    std::mt19937 rng_;
};

[[nodiscard]] auto is_opposite(Direction lhs, Direction rhs) -> bool;
[[nodiscard]] auto next_cell(GridPos head, Direction direction) -> GridPos;

}  // namespace game_core
