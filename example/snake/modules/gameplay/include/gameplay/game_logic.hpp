#pragma once

#include "game_core/game_state.hpp"

namespace gameplay {

// Pure gameplay rules: apply turn input and advance one simulation tick.
class GameLogic {
public:
    GameLogic() = default;

    auto request_turn(game_core::Direction direction) -> void;
    auto tick(game_core::GameState& state) -> void;

private:
    game_core::Direction queued_direction_{game_core::Direction::Right};
    bool has_queued_direction_{false};
};

}  // namespace gameplay
