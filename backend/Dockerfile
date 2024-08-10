# using ubuntu LTS version
FROM ubuntu:22.04 AS builder-image

# avoid stuck build due to user prompt
ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install --no-install-recommends -y python3.11 python3.11-dev python3.11-venv python3-pip python3-wheel build-essential libxml2-dev libxslt1-dev zlib1g-dev libffi-dev libssl-dev && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# create and activate virtual environment
# using final folder name to avoid path issues with packages
RUN python3.11 -m venv /home/myuser/venv
ENV PATH="/home/myuser/venv/bin:$PATH"

# install requirements
COPY requirements.txt .
RUN pip3 install --no-cache-dir wheel
RUN pip3 install --no-cache-dir -r requirements.txt

FROM ubuntu:22.04 AS runner-image
RUN apt-get update && apt-get install --no-install-recommends -y python3.11 python3-venv libxml2-dev libxslt1-dev zlib1g-dev libffi-dev libssl-dev && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home myuser
COPY --from=builder-image /home/myuser/venv /home/myuser/venv

USER myuser
RUN mkdir /home/myuser/code
WORKDIR /home/myuser/code
COPY . .

# make sure all messages always reach console
ENV PYTHONUNBUFFERED=1

# activate virtual environment
ENV VIRTUAL_ENV=/home/myuser/venv
ENV PATH="/home/myuser/venv/bin:$PATH"

CMD ["uvicorn", "app.main:app", "--proxy-headers", "--host", "0.0.0.0", "--port", "6969", "--reload"]
