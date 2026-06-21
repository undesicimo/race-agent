# Changelog

## [2.0.0](https://github.com/undesicimo/race-agent/compare/v1.0.1...v2.0.0) (2026-06-20)


### ⚠ BREAKING CHANGES

* Renames package scopes, app identifiers, and local database defaults from sim-telemetry to race-agent.

### Bug Fixes

* Add app and web logos; set Windows app icon ([10a127d](https://github.com/undesicimo/race-agent/commit/10a127d191282a6d92deb3c40adad39ea74a6e9d))
* **commitlint:** allow long body and footer lines ([d731d45](https://github.com/undesicimo/race-agent/commit/d731d454dabcff518a93a85983caa961cc207d93))
* **commitlint:** disable body and footer line-length limits ([3f55c29](https://github.com/undesicimo/race-agent/commit/3f55c291a3163198703117cc68b3f6629f65ea8d))
* rename sim-telemetry to race-agent ([8c16293](https://github.com/undesicimo/race-agent/commit/8c16293508691185dd3513cbd7b673a4afa1c0f8))
* Revamp UI styles and dashboard layout ([542bb00](https://github.com/undesicimo/race-agent/commit/542bb0081217d23b491523075977719522a272f5))

## [1.0.1](https://github.com/undesicimo/race-agent/compare/v1.0.0...v1.0.1) (2026-06-20)


### Bug Fixes

* verify windows collector releases ([59609d4](https://github.com/undesicimo/race-agent/commit/59609d41095ae299ded1417252049fd28fe38f4b))

## 1.0.0 (2026-06-20)


### Features

* add check-docker script to ensure Docker is installed and running before starting the database ([ff900bd](https://github.com/undesicimo/race-agent/commit/ff900bd3396fc7bd73341f0cb8e38b68967bed99))
* add release please collector releases ([7e528fd](https://github.com/undesicimo/race-agent/commit/7e528fd4fadb24e0e9ebd5b12efc59478aadf7fd))
* **collector-windows:** implement Windows tray application with settings UI ([745548e](https://github.com/undesicimo/race-agent/commit/745548e4ba906806cd7c2e75946b9caa474e820b))
* complete initial implementation of ACC telemetry collector and related components ([9c6a1c3](https://github.com/undesicimo/race-agent/commit/9c6a1c37574b3903a45bcd0cd6f6fa51adc1ec50))
* **database:** add apikey and collectorHeartbeats tables, update cars and tracks with unique indexes ([d3564bc](https://github.com/undesicimo/race-agent/commit/d3564bc55d8dfec342aa48888a0d945151fb4303))
* enhance documentation and setup for local development with Docker and TimescaleDB ([ccd7984](https://github.com/undesicimo/race-agent/commit/ccd7984087710a0b87684347f76be67f206f7754))
* implement TokenCreator component for generating and managing collector tokens ([5c7c76b](https://github.com/undesicimo/race-agent/commit/5c7c76b224caed688e5472561154e0b6f51ea433))
* improve error handling for token creation and validate DATABASE_URL ([b6957a7](https://github.com/undesicimo/race-agent/commit/b6957a7317fc3a88bdb0c7948b87e97202c5405b))
* refactor authentication and database access, add oxlint configuration ([f6ce8d6](https://github.com/undesicimo/race-agent/commit/f6ce8d66d2dc30ae1582de07a4e26d4f554aebbc))
* update TODO structure and add next steps for development ([f7c25cf](https://github.com/undesicimo/race-agent/commit/f7c25cfaa2c735e866081da3889ae90b70bade47))
