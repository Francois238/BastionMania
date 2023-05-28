import { UserBastionInfo } from "./user-bastion-info";

export interface ListUserBastionReceived {
    success: boolean;
    message: string;
    data: UserBastionInfo[];
}
