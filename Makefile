main: main.cpp
	g++ -std=c++17 main.cpp -I$(HOME)/VulkanSDK/1.3.275.0/macOS/include -I/opt/homebrew/include -L/opt/homebrew/lib -L$(HOME)/VulkanSDK/1.3.275.0/macOS/lib `pkg-config --static --libs glfw3 vulkan` -lglfw -lvulkan --verbose