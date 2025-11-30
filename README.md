# [프로젝트 개요](https://narrow-factory-9d4.notion.site/CryptoStreaming-017c6f53e108488f94c2e3962932e104)


## 프로젝트 목표

실시간으로 다양한 암호화폐 거래소(Binance, Bithumb, Upbit, Kucoin)에서 진행되는 거래 및 OHLCV 데이터를 통해 거래소 간 발생하는 매매차익을 기반으로 지표 생성 및 대시보드를 통해 시장 현황을 추적

## [API 명세서](https://documenter.getpostman.com/view/27584637/2sAXjKZXov#intro)
| 거래소 명          | API 명세서                                                                                                                                                                                                                                                                                                         |
|---------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Binance      |  [<img src="https://img.shields.io/badge/Postman-FF6C37?style=flat&logo=postman&logoColor=white"/>](https://documenter.getpostman.com/view/27584637/2sAXjKZXov#b547fa42-6070-488e-9c46-52a83d084d24)|      
| Kucoin      |  [<img src="https://img.shields.io/badge/Postman-FF6C37?style=flat&logo=postman&logoColor=white"/>](https://documenter.getpostman.com/view/27584637/2sAXjKZXov#dd9fe277-b6c5-4133-9dcc-fb167abf2552)|  
| Upbit      |  [<img src="https://img.shields.io/badge/Postman-FF6C37?style=flat&logo=postman&logoColor=white"/>](https://documenter.getpostman.com/view/27584637/2sAXjKZXov#8a0778fa-7e8d-40f2-961d-7ee54de98141)|  
| Bithumb      |  [<img src="https://img.shields.io/badge/Postman-FF6C37?style=flat&logo=postman&logoColor=white"/>](https://documenter.getpostman.com/view/27584637/2sAXjKZXov#4e422bbd-10cf-48e2-9100-995603f0eef8)|  

## 기술 스택

| Role          | Stack                                                                                                                                                                                                                                                                                                        |
|---------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Language      |  <img src="https://img.shields.io/badge/Rust-DC322F?style=flat&logo=rust&logoColor=black"/> <img src="https://img.shields.io/badge/Python-3776AB?style=flat&logo=python&logoColor=yellow"/> |                                                                                                                                                                                           
| Streaming | <img src="https://img.shields.io/badge/Kafka-231F20?style=flat&logo=Apachekafka&logoColor=white"/> <img src="https://img.shields.io/badge/Spark-E25A1C?style=flat&logo=apache spark&logoColor=white"/> <img src="https://img.shields.io/badge/MySQL-4479A1?style=flat&logo=Mysql&logoColor=white"/> <img src="https://img.shields.io/badge/Grafana-F46800?style=flat&logo=Grafana&logoColor=white"/>      |

## 프로젝트 아키텍처
![image](https://github.com/user-attachments/assets/9e70f1df-94ea-4c8a-b452-b95591e6845e)

## 프로젝트 디렉토리 구조
```
📦producer
 ┣ 📂src
 ┃ ┣ 📂api               # 각 암호화폐 거래소 API와 관련된 코드
 ┃ ┃ ┣ 📜binance.rs      # Binance API 호출 및 데이터 처리 로직
 ┃ ┃ ┣ 📜bithumb.rs      # Bithumb API 호출 및 데이터 처리 로직
 ┃ ┃ ┣ 📜kucoin.rs       # KuCoin API 호출 및 데이터 처리 로직
 ┃ ┃ ┣ 📜upbit.rs        # Upbit API 호출 및 데이터 처리 로직
 ┃ ┃ ┗ 📜mod.rs
 ┃ ┣ 📂kafka             # Kafka와의 통신을 위한 코드
 ┃ ┃ ┣ 📜producer.rs     # Kafka producer 생성 및 메시지 전송 로직
 ┃ ┃ ┗ 📜mod.rs
 ┃ ┣ 📂models            # 애플리케이션의 데이터 구조를 정의하는 코드
 ┃ ┃ ┣ 📜crypto_symbols.rs # 암호화폐 심볼 관련 구조체 정의
 ┃ ┃ ┣ 📜market_data.rs  # 암호화폐 가격을 포함한 OHLCV 데이터 관련 구조체 정의
 ┃ ┃ ┗ 📜mod.rs          
 ┃ ┣ 📂services          # 비즈니스 로직을 처리하는 서비스 계층
 ┃ ┃ ┣ 📜service.rs      # 데이터 수집 및 Kafka 전송 서비스를 위한 로직       
 ┃ ┃ ┗ 📜mod.rs 
 ┃ ┣ 📜main.rs           # 프로그램의 진입점, 주요 로직 실행
 ┃ ┗ 📜utils.rs          # 유틸리티 함수 (파싱, 심볼 정규화 등)를 포함한 헬퍼 함수들
```
```
📦consumer
 ┣ 📂config
 ┃ ┗ 📜config.py             # 설정 파일. Kafka, MySQL 등의 설정값을 관리
 ┣ 📂src
 ┃ ┣ 📂processing
 ┃ ┃ ┣ 📜data_processing.py  # 매매차익 관련 계산과 같은 핵심 데이터 처리 로직을 담은 모듈
 ┃ ┃ ┗ 📜fees.py             # 각 암호화폐 거래소의 거래 수수료 관련 정보를 담은 모듈
 ┃ ┣ 📂storage
 ┃ ┃ ┗ 📜mysql_sink.py       # MySQL에 데이터를 저장하는 로직을 담은 모듈
 ┃ ┣ 📂streaming
 ┃ ┃ ┣ 📜kafka_schema.py     # Kafka 메시지의 스키마 정의 모듈
 ┃ ┃ ┗ 📜streaming_job.py    # Kafka 스트림을 처리하는 메인 로직을 담은 모듈
 ┃ ┗ 📜main.py               # Consumer 서비스의 메인 엔트리 포인트
 ┗ 📜utils.py                # 공통 유틸리티 함수 모음
```

# 프로젝트 결과
## Structured Streaming (feat. Kafka, Spark)
https://github.com/user-attachments/assets/d72d4b4c-52a3-4b1f-85a2-767b05dee7c6

## MySQL 
crypto_prices|arbitrage_average|arbitrage_values|
|------|---|---|
|<img width="797" alt="crypto_prices" src="https://github.com/user-attachments/assets/435edd0a-83bc-4731-8a78-fca98cbd340f">|<img width="523" alt="arbitrage_average" src="https://github.com/user-attachments/assets/76d0d0e6-e94a-47d3-832a-02bedd2386f4">|<img width="387" alt="arbitrage_values" src="https://github.com/user-attachments/assets/baabbe76-b095-435c-852f-08ee7cc54d67">|


## Grafana
BTC 실시간 매매차익 분석 |ETH 실시간 매매차익 분석|
|------|------|
|![BTC](https://github.com/user-attachments/assets/a46b06ab-ce5c-40b6-b7c0-96fc12765062)|![ETH](https://github.com/user-attachments/assets/a6514347-89d9-44ec-8881-f7d0d32e577b)|

암호화폐 시장 현황 분석|Slack 알람 시스템|
|------|------|
|![overall](https://github.com/user-attachments/assets/86aea59c-d679-4545-9e03-582ba48a0918)|![grafana_slack_alarm](https://github.com/user-attachments/assets/e0ba79d2-c503-4528-938a-73f665743692)|

# 프로젝트 실행 
## 요구사항
이 프로젝트를 실행하려면 다음이 필요합니다:
- [Rust](https://www.rust-lang.org/tools/install) (build producer)
- [Python 3.x](https://www.python.org/downloads/) (execute consumer)
- [Java](https://www.oracle.com/kr/java/technologies/downloads/) (over ver.8), [JDBC connector](https://dev.mysql.com/downloads/connector/j/) (spark-mysql connection)
- [MySQL](https://dev.mysql.com/downloads/installer/), [Grafana](https://grafana.com/docs/grafana/latest/setup-grafana/installation/) 설치 및 호스팅 환경
- [Docker Desktop](https://docs.docker.com/desktop/install/mac-install/), [Docker Compose](https://docs.docker.com/compose/install/) 설치 (만약 Docker를 사용하지 않고 이미 구축되어 있는 Kafka와 Spark 실행 환경을 사용할 경우, docker-compose.yml을 참고하여 버젼 관리에 유의하여 진행)

## 프로젝트 세팅
### 1. 프로젝트 클론 및 종속성 설치
```bash
git clone https://github.com/idle-danie/crypto-trading-profit.git
cd crypto-trading-profit
```
### **2. env.sample을 참고하여 환경변수 설정**
- [producer/.env.sample](https://github.com/idle-danie/crypto-trading-profit/blob/develop/producer/.env.sample)
- [consumer/.env.sample](https://github.com/idle-danie/crypto-trading-profit/blob/develop/consumer/.env.sample)

### **3. Docker Compose를 사용한 서비스 실행**
Rust를 설치한 후, producer 애플리케이션을 실행합니다:
```bash
docker-compose up -d
```

### **4. Kafka producer 실행**
Rust를 설치한 후, producer 애플리케이션을 실행합니다:
```bash
cd producer
cargo run
```

### **5. (Optional) Python 가상환경 설치 및 실행**
```bash 
python3 -m venv venv
source venv/bin/activate
```

### **6. Python 실행 환경 세팅 및 종속성 설치**
```bash
export PYTHONPATH=$PYTHONPATH:/Users/xxx/crypto-trading-profit/consumer
cd consumer
pip install -r requirements.txt
```

### **7. Spark consumer 실행**
```bash 
spark-submit --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.5.2 \
  --conf spark.driver.bindAddress=127.0.0.1 \
  --conf spark.driver.host=127.0.0.1 \
  --jars /path/to/mysql-connector-j-9.0.0.jar \
  src/main.py
```
