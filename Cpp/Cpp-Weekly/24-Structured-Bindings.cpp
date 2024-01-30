// 23: structured bindings
#include <map>
#include <utility>

int &add_local(std::map<std::string, int> vars, const std::string &new_var_name) {
  // use structured binding in combo with init if statement
  if (auto [itr, success] = vars.insert(std::make_pair(new_var_name, 0)); success == false) {
    throw std::runtime_error("variable already exists");
  } else {
    auto [key, value] = *itr;
    return value;
  }
}

int main() {
  std::map<std::string, int> locals;
  int &i = add_local(locals, "my_var");
  i = 5;
  return 0;
}
