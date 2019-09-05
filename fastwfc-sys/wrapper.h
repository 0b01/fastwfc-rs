#include "fastwfc/overlapping_wfc.hpp"
#include "fastwfc/tiling_wfc.hpp"

/**
 * Represent a 24-bit rgb color.
 */
struct Color {
  unsigned char r, g, b;
  bool operator==(const Color &c) const noexcept {
    return r == c.r && g == c.g && b == c.b;
  }
  bool operator!=(const Color &c) const noexcept { return !(c == *this); }
};
/**
 * Hash function for color.
 */
namespace std {
template <> class hash<Color> {
public:
  size_t operator()(const Color &c) const {
    return (size_t)c.r + (size_t)256 * (size_t)c.g +
           (size_t)256 * (size_t)256 * (size_t)c.b;
  }
};
} // namespace std

class ArrayColor2D {
public:
    Array2D<Color> inner;
    void set_width(int width) { inner.width = width; }
    void set_height(int height) { inner.height = height; }
    void set_inner(void* ptr) { inner.data = *(std::vector<Color>*)ptr; }
};

std::optional<Array2D<Color>> run_overlapping(ArrayColor2D *m, OverlappingWFCOptions options, int seed) {
    OverlappingWFC<Color> wfc(m->inner, options, seed);
    std::optional<Array2D<Color>> success = wfc.run();
    return success;
}