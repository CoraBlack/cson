#include "settings/game_settings.hpp"

#include <cctype>
#include <stdexcept>

namespace {

auto to_lower_copy(std::string_view text) -> std::string {
    std::string result(text.begin(), text.end());
    for (auto& ch : result) {
        ch = static_cast<char>(std::tolower(static_cast<unsigned char>(ch)));
    }

    return result;
}

}  // namespace

namespace settings {

auto tick_interval_ms(const Difficulty difficulty) -> int {
    switch (difficulty) {
        case Difficulty::Easy:
            return 180;
        case Difficulty::Normal:
            return 130;
        case Difficulty::Hard:
            return 95;
    }

    throw std::runtime_error("Unsupported difficulty");
}

auto board_columns(const BoardPreset preset) -> int {
    switch (preset) {
        case BoardPreset::Compact:
            return 24;
        case BoardPreset::Classic:
            return 32;
        case BoardPreset::Wide:
            return 44;
    }

    throw std::runtime_error("Unsupported board preset");
}

auto board_rows(const BoardPreset preset) -> int {
    switch (preset) {
        case BoardPreset::Compact:
            return 18;
        case BoardPreset::Classic:
            return 24;
        case BoardPreset::Wide:
            return 24;
    }

    throw std::runtime_error("Unsupported board preset");
}

auto render_mode_name(const RenderMode mode) -> std::string {
    switch (mode) {
        case RenderMode::Pixel:
            return "Pixel";
        case RenderMode::Retro:
            return "Retro";
        case RenderMode::Neon:
            return "Neon";
    }

    throw std::runtime_error("Unsupported render mode");
}

auto make_settings_from_mode(const std::string_view mode) -> GameSettings {
    const std::string normalized = to_lower_copy(mode);

    if (normalized == "casual") {
        GameSettings settings;
        settings.difficulty = Difficulty::Easy;
        settings.board = BoardPreset::Classic;
        settings.render = RenderMode::Retro;
        settings.seed = 1001;
        settings.show_grid = true;
        settings.wrap_world = true;
        return settings;
    }

    if (normalized == "arcade") {
        GameSettings settings;
        settings.difficulty = Difficulty::Hard;
        settings.board = BoardPreset::Compact;
        settings.render = RenderMode::Neon;
        settings.seed = 424242;
        settings.show_grid = false;
        settings.wrap_world = false;
        return settings;
    }

    if (normalized == "zen") {
        GameSettings settings;
        settings.difficulty = Difficulty::Normal;
        settings.board = BoardPreset::Wide;
        settings.render = RenderMode::Pixel;
        settings.seed = 2026;
        settings.show_grid = true;
        settings.wrap_world = true;
        return settings;
    }

    return GameSettings{};
}

auto mode_help_text() -> std::string {
    return "casual: easy speed + wrap, arcade: fast compact map, zen: wide map with stable pace";
}

}  // namespace settings
