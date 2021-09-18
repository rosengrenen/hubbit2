.PHONY: load_gamma create_gamma_client

load_gamma: gamma.sql
	echo "DROP SCHEMA public CASCADE" | docker exec -i hubbit2_gamma-db_1 psql -U gamma
	echo "CREATE SCHEMA public" | docker exec -i hubbit2_gamma-db_1 psql -U gamma
	docker exec -i hubbit2_gamma-db_1 psql -U gamma gamma < gamma.sql
	echo "UPDATE ituser SET password='{bcrypt}$2y$10$6Bou8FiSMDtt1O3j1BCxF.q1LAOA1YHnMsFyj7zGooOICBfBCFFb2' WHERE cid='admin'" | docker exec -i hubbit2_gamma-db_1 psql -U gamma

create_gamma_client:
	echo "INSERT INTO internal_text (id, sv, en) VALUES ('dc989ea3-c80b-4fbf-97d3-dbb6869cdd26', 'hubbit', 'hubbit')" | docker exec -i hubbit2_gamma-db_1 psql -U gamma gamma
	echo "INSERT INTO itclient (id, client_id, client_secret, web_server_redirect_uri, access_token_validity, refresh_token_validity, auto_approve, name, description) VALUES ('714ee306-e904-4978-bb2b-cd1a3478062c', 'hubbit', '{noop}hubbit', 'http://localhost:8080/api/auth/gamma/callback', 3600, 500000000, true, 'hubbit', 'dc989ea3-c80b-4fbf-97d3-dbb6869cdd26')"  | docker exec -i hubbit2_gamma-db_1 psql -U gamma gamma
	echo "INSERT INTO apikey (id, name, description, key) VALUES ('c0f26d1b-9e70-4218-bb58-62ba2da72ce5', 'hubbit', 'dc989ea3-c80b-4fbf-97d3-dbb6869cdd26', 'hubbit')"  | docker exec -i hubbit2_gamma-db_1 psql -U gamma gamma 

setup_gamma: load_gamma create_gamma_client

setup_hubbit:
	cd backend && cargo sqlx migrate run

setup: setup_gamma setup_hubbit
