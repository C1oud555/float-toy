#pragma once

#include "state_interface.hpp"

namespace E4M3 {

struct StateE4M3 : public AppInterface {
  constexpr ftxui::Component create_component() override;
  constexpr std::string get_format_string() override;

private:
};

constexpr ftxui::Component StateE4M3::create_component() {
  int value = 0;
  auto buttons = ftxui::Container::Horizontal({ftxui::Button(
      "123", [&] { value--; },
      ftxui::ButtonOption::Animated(ftxui::Color::Red))});

  auto component = Renderer(buttons, [&] {
    return ftxui::vbox({
        ftxui::vbox({
            ftxui::text("value = " + std::to_string(value)),
            ftxui::separator(),
            ftxui::gauge(value * 0.01f),
        }) | ftxui::border,
        buttons->Render(),
    });
  });

  return component;
}
} // namespace E4M3
