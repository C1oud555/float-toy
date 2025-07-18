add_rules("mode.debug", "mode.release")

add_requires("ftxui")
add_languages("cxx26")
add_packages("ftxui")

-- stylua: ignore start
target("float-toy")
    set_kind("binary")
    add_files("src/*.cpp")
