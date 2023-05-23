import { Component, EventEmitter, Input, Output } from '@angular/core';
import { BastionInfo } from '../bastion-info';

@Component({
  selector: 'app-list-user-resource-detail',
  templateUrl: './list-user-resource-detail.component.html',
  styleUrls: ['./list-user-resource-detail.component.scss']
})
export class ListUserResourceDetailComponent {

  @Input() user!: BastionInfo;
  @Output() newItemEvent = new EventEmitter<string>();

}
