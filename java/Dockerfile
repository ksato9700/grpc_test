FROM amazoncorretto:21
WORKDIR /app
COPY . .
RUN ./gradlew installDist
CMD ["app/build/install/app/bin/hello-world-server"]
