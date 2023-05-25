import { Component, EventEmitter, Input, Output } from '@angular/core';
import { BastionInfo } from '../bastion-info';
import { UserBastionInfo } from '../user-bastion-info';

@Component({
  selector: 'app-list-user-resource-detail',
  templateUrl: './list-user-resource-detail.component.html',
  styleUrls: ['./list-user-resource-detail.component.scss']
})
export class ListUserResourceDetailComponent {

  @Input() user!: UserBastionInfo;
  @Output() newItemEvent = new EventEmitter<string>();

}
