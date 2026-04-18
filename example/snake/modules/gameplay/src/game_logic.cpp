#include "gameplay/game_logic.hpp"

#include <cstddef>

namespace gameplay {

auto GameLogic::request_turn(const game_core::Direction direction) -> void {
    queued_direction_ = direction;
    has_queued_direction_ = true;
}

auto GameLogic::tick(game_core::GameState& state) -> void {
    if (state.is_game_over()) {
        return;
    }

    game_core::Direction next_direction = state.direction();

    if (has_queued_direction_ && !game_core::is_opposite(state.direction(), queued_direction_)) {
        next_direction = queued_direction_;
        state.set_direction(next_direction);
    }

    has_queued_direction_ = false;

    game_core::GridPos candidate =
        game_core::next_cell(state.snake().front(), next_direction);

    if (state.settings().wrap_world) {
        if (candidate.x < 0) {
            candidate.x = state.cols() - 1;
        } else if (candidate.x >= state.cols()) {
            candidate.x = 0;
        }

        if (candidate.y < 0) {
            candidate.y = state.rows() - 1;
        } else if (candidate.y >= state.rows()) {
            candidate.y = 0;
        }
    } else {
        if (candidate.x < 0 || candidate.y < 0 || candidate.x >= state.cols() ||
            candidate.y >= state.rows()) {
            state.set_game_over(true);
            return;
        }
    }

    const bool eat_food = candidate == state.food();

    bool hits_body = false;
    const auto& snake = state.snake();

    // The head is at index 0, so start from index 1 for body collision.
    for (std::size_t i = 1; i < snake.size(); ++i) {
        if (snake[i] == candidate) {
            // Moving into current tail is valid only when not eating,
            // because tail moves away in the same tick.
            if (!eat_food && i == snake.size() - 1) {
                continue;
            }

            hits_body = true;
            break;
        }
    }

    if (hits_body) {
        state.set_game_over(true);
        return;
    }

    state.push_head(candidate);

    if (!eat_food) {
        state.pop_tail();
    }

    if (eat_food) {
        state.add_score(10);
        state.set_food(state.random_free_cell());
    }
}

}  // namespace gameplay
