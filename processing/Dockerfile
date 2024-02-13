FROM ubuntu:20.04

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update -y
RUN apt-get install -y \
    git \
    cmake \
    libsm6 \
    libxext6 \
    libxrender-dev \
    python3 \
    python3-pip \
    gcc \
    python3-tk \
    ffmpeg \
    libopenblas-dev \
    liblapack-dev

# Install dlib
RUN git clone https://github.com/davisking/dlib.git && \
    cd dlib && \
    mkdir build && \
    cd build && \
    cmake .. && \
    cmake --build . && \
    cd .. && \
    python3 setup.py install

WORKDIR /app

RUN pip3 install --upgrade pip

COPY requirements.txt .
RUN pip3 install --no-cache-dir -r requirements.txt

COPY main.py .

ENTRYPOINT ["python3", "-u", "main.py"]
