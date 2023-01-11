import { TestBed, inject } from "@angular/core/testing";

import { LocalStorage } from "./local-storage.service";

describe("LocalStorageService", () => {
	beforeEach(() => {
		TestBed.configureTestingModule({
			providers: [LocalStorage]
		});
	});

	it("should be created", inject([LocalStorage], (service: LocalStorage) => {
		expect(service).toBeTruthy();
	}));
});
