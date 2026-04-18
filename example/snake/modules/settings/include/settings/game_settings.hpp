#pragma once

#include <cstdint>
#include <string>
#include <string_view>

namespace settings {

// Difficulty controls snake tick interval.
enum class Difficulty : std::uint8_t {
    Easy,
    Normal,
    Hard
};

// BoardPreset controls map width and height.
enum class BoardPreset : std::uint8_t {
    Compact,
    Classic,
    Wide
};

// RenderMode switches visual themes.
enum class RenderMode : std::uint8_t {
    Pixel,
    Retro,
    Neon
};

struct GameSettings {
    Difficulty difficulty{Difficulty::Normal};
    BoardPreset board{BoardPreset::Classic};
    RenderMode render{RenderMode::Pixel};
    std::uint32_t seed{12345};
    bool show_grid{true};
    bool wrap_world{false};
};

// Convert gameplay parameters into runtime values.
[[nodiscard]] auto tick_interval_ms(Difficulty difficulty) -> int;
[[nodiscard]] auto board_columns(BoardPreset preset) -> int;
[[nodiscard]] auto board_rows(BoardPreset preset) -> int;

// Build human-readable labels and preset bundles.
[[nodiscard]] auto render_mode_name(RenderMode mode) -> std::string;
[[nodiscard]] auto make_settings_from_mode(std::string_view mode) -> GameSettings;
[[nodiscard]] auto mode_help_text() -> std::string;

}  // namespace settings
