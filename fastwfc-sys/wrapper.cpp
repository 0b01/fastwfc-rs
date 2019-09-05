#include "overlapping_wfc.hpp"
#include "direction.hpp"
#include "propagator.hpp"
#include "tiling_wfc.hpp"
#include "wave.hpp"
#include "wfc.hpp"

#include <random>

struct Color {
  unsigned char r, g, b, a;
  bool operator==(const Color &c) const noexcept {
    return r == c.r && g == c.g && b == c.b && a == c.a;
  }
  bool operator!=(const Color &c) const noexcept { return !(c == *this); }
};
namespace std {
template <> class hash<Color> {
public:
  size_t operator()(const Color &c) const {
    return (size_t)c.r + (size_t)256 * (size_t)c.g +
           (size_t)256 * (size_t)256 * (size_t)c.b +
           (size_t)256 * (size_t)256 * (size_t)256 * (size_t)c.a;
  }
};
}

int get_random_seed() {
  #ifdef __linux__
    return std::random_device()();
  #else
    return rand();
  #endif
}

class ArrayColor2D {
public:
  ArrayColor2D(): init(false), inner(0,0), ref(nullptr) {}
  ArrayColor2D(Array2D<Color> inner): init(true), inner(inner), ref(nullptr) {}

  bool init;
  Array2D<Color> inner;
  void* ref;
};

ArrayColor2D* new_array_color_2d() { return new ArrayColor2D(); }
void array_color_2d_set_width(ArrayColor2D *c, int width) { c->inner.width = width; }
void array_color_2d_set_height(ArrayColor2D *c, int height) { c->inner.height = height; }
void array_color_2d_set_data(ArrayColor2D *c, Color* ptr, void* ref) {
  c->inner.data.assign(ptr, ptr + c->inner.width * c->inner.height);
  c->ref = ref;
}
void destroy_array_color_2d(ArrayColor2D* c) {delete c;}

int array_color_2d_get_width(ArrayColor2D *c) { return c->inner.width; }
int array_color_2d_get_height(ArrayColor2D *c) { return c->inner.height; }
void* array_color_2d_get_data(ArrayColor2D *c) { return c->inner.data.data(); }
void* array_color_2d_get_ref(ArrayColor2D *c) { return c->ref; }

ArrayColor2D* run_overlapping(ArrayColor2D *m, OverlappingWFCOptions options, unsigned tries) {
  for (unsigned i = 0; i < tries; i++) {
    int seed = get_random_seed();
    OverlappingWFC<Color> wfc(m->inner, options, seed);
    std::optional<Array2D<Color>> success = wfc.run();
    if (success) return new ArrayColor2D(*success);
  }
  return new ArrayColor2D();
}

ArrayColor2D* run_overlapping_with_seed(ArrayColor2D *m, OverlappingWFCOptions options, int seed) {
  OverlappingWFC<Color> wfc(m->inner, options, seed);
  std::optional<Array2D<Color>> success = wfc.run();
  if (success) return new ArrayColor2D(*success);
  return new ArrayColor2D();
}