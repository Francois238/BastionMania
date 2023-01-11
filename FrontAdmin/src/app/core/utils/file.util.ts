import { Injectable } from "@angular/core";
import { NotificationComponent } from "../../shared/notification/notification.component";

@Injectable()
export class FileUtil {
	constructor(protected notification: NotificationComponent) {}

	isCSVFile(file) {
		return file.name.endsWith(".csv");
	}

	getHeaderArray(csvRecordsArr, tokenDelimeter) {
		const headers = csvRecordsArr[0].split(tokenDelimeter);
		const headerArray = [];

		for (const header of headers) {
			headerArray.push(header);
		}
		return headerArray;
	}

	validateHeaders(origHeaders, fileHeaaders) {
		if (origHeaders.length !== fileHeaaders.length) {
			return false;
		}

		let fileHeaderMatchFlag = true;
		for (let j = 0; j < origHeaders.length; j++) {
			if (origHeaders[j] !== fileHeaaders[j]) {
				fileHeaderMatchFlag = false;
				break;
			}
		}
		return fileHeaderMatchFlag;
	}

	getDataRecordsArrayFromCSVFile(
		csvRecordsArray,
		headerLength,
		validateHeaderAndRecordLengthFlag,
		tokenDelimeter
	) {
		const dataArr = [];

		for (let i = 0; i < csvRecordsArray.length; i++) {
			const data = csvRecordsArray[i].split(tokenDelimeter);

			if (
				validateHeaderAndRecordLengthFlag &&
				data.length !== headerLength
			) {
				if (data === "") {
					this.notification.setErrors(
						`Extra blank line is present at line number ${i},
						please remove it.`
					);
				} else {
					this.notification
						.setErrors(`Record at line number ${i} contain ${data.length} tokens,
						and is not matching with header length of : ${headerLength}`);
				}
				this.notification.showError();
				return null;
			}

			const col = [];
			for (const oneData of data) {
				col.push(oneData);
			}

			dataArr.push(col);
		}
		return dataArr;
	}
}
