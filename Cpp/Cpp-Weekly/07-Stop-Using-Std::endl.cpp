// 7: Stop using std::endl
#include <iostream>
#include <fstream>

void writeEndl(std::ostream &os, const std::string &str) {
  os << str << std::endl;
}

void writeNewl(std::ostream &os, const std::string &str) {
  os << str << '\n';
}

int main() {
  // this is equivalent 
  // std::cout << "hello world" << std::endl;
  // to this
  // std::cout << "hello world" << '\n' << std::flush;
  // do you really need to be doing a flush??
  // could do it once at the very end if we want to make sure it is flushed

  std::ofstream outfile("output.txt", std::ios_base::trunc);

  // this takes .74 seconds
  for(int i = 0; i < 1000000; i++) {
    writeEndl(outfile, "hello world");
  }

  // this takes .04 seconds
  for(int i = 0; i < 1000000; i++) {
    writeNewl(outfile, "hello world");
  }
}

