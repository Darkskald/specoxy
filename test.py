from specoxy import Spec2d
import specoxy
import matplotlib.pyplot as plt
import matplotlib

spectrum = specoxy.deliver_ir()

plt.plot(spectrum.x, spectrum.y)
plt.xlabel(spectrum.x_label)
plt.ylabel(spectrum.y_label)

plt.show()