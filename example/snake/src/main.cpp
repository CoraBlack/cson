#include "app/snake_app.hpp"
#include "settings/game_settings.hpp"

#include <exception>
#include <iostream>
#include <string>

int main(int argc, char** argv) {
    // Default to a friendly mode if user does not pass arguments.
    std::string mode = "casual";

    if (argc >= 2) {
        mode = argv[1];
    }

    try {
        // Parse one-word mode into a structured settings object.
        auto settings = settings::make_settings_from_mode(mode);
        app::SnakeApp app(settings);
        return app.run();
    } catch (const std::exception& ex) {
        std::cerr << "Failed to run SnakeSDL3: " << ex.what() << '\n';
        std::cerr << "Try mode: " << settings::mode_help_text() << '\n';
        return 1;
    }
}
