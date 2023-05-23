import { Component, EventEmitter, Input, Output } from '@angular/core';
import { BastionInfo } from '../bastion-info';

@Component({
  selector: 'app-bastion-detail',
  templateUrl: './bastion-detail.component.html',
  styleUrls: ['./bastion-detail.component.scss']
})
export class BastionDetailComponent {

  @Input() bastion!: BastionInfo;
  @Output() newItemEvent = new EventEmitter<string>();


}
