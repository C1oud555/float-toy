#pragma once

#include "ftxui/component/component_options.hpp"
#include <ftxui/component/component.hpp>
#include <ftxui/component/component_base.hpp>
#include <ftxui/dom/elements.hpp>
#include <ftxui/screen/screen.hpp>

struct AppInterface {
public:
  virtual constexpr ftxui::Component create_component() = 0;
  virtual constexpr std::string get_format_string() = 0;
};
