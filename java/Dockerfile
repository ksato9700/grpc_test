FROM amazoncorretto:25
WORKDIR /app
COPY . .
RUN ./gradlew installDist
CMD ["app/build/install/app/bin/hello-world-server"]
