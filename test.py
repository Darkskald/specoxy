from specoxy import Spec2d
import specoxy
import matplotlib.pyplot as plt
import matplotlib

spectrum = specoxy.deliver_ir("/home/flo/data/linux_data/rust_development/specoxy/BX12.dpt")

plt.plot(spectrum.x, spectrum.y)
plt.xlabel(spectrum.x_label)
plt.ylabel(spectrum.y_label)

plt.show()